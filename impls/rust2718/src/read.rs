/*!
Reading input for the interpreter.
*/
use std::sync::{
    mpsc::{channel, Receiver, Sender},
    Arc,
};

use once_cell::sync::Lazy;
use regex::{bytes, Regex};
use tracing::{event, instrument, Level};

use crate::{
    env::Env,
    error::rerr,
    eval::eval,
    types::{List, Map},
    MalErr, Res, Val,
};

static TOKENIZER: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"[\s,]*(~@|[\[\]{}()'`~^@]|"(?:\.|[^\"])*"?|;.*|[^\s\[\]{}('",;)]*)"#)
        .expect("unable to init tokenizing regex")
});
static ESCAPER: Lazy<bytes::Regex> =
    Lazy::new(|| bytes::Regex::new(r#"\\"#).expect("unable to init escaping regex"));

#[derive(Debug, PartialEq)]
pub enum Token {
    OpenParen,
    OpenBracket,
    OpenBrace,
    CloseParen,
    CloseBracket,
    CloseBrace,
    SingleQuote,
    Comment(String),
    Obj(String),
}

impl From<&str> for Token {
    fn from(s: &str) -> Token {
        match s {
            "(" => Token::OpenParen,
            ")" => Token::CloseParen,
            "[" => Token::OpenBracket,
            "]" => Token::CloseBracket,
            "{" => Token::OpenBrace,
            "}" => Token::CloseBrace,
            "'" => Token::SingleQuote,
            other => {
                if other.as_bytes().first() == Some(&b';') {
                    Token::Comment(other.to_string())
                } else {
                    Token::Obj(other.to_string())
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct Tokenizer {
    input: rustyline::DefaultEditor,
    output: Sender<Token>,
}

impl Tokenizer {
    #[instrument]
    pub fn read_line(&mut self) {
        match self.input.readline("user> ") {
            Ok(line) => self.tokenize(line),
            Err(e) => {
                println!("exiting: {}", &e);
                std::process::exit(0);
            }
        }
    }

    fn tokenize(&mut self, line: String) {
        for tok in TOKENIZER.captures_iter(&line).filter_map(|t| {
            match t.get(1).map(|m| m.as_str().trim()) {
                None | Some("") => None,
                Some(s) => Some(Token::from(s)),
            }
        }) {
            if !matches!(&tok, &Token::Comment(_)) {
                self.output.send(tok).unwrap();
            }
        }
    }
}

#[derive(Debug)]
pub struct Reader {
    input: Receiver<Token>,
    current: Option<Token>,
}

impl Reader {
    pub fn peek(&mut self) -> &Token {
        if self.current.is_none() {
            let tok = self.input.recv().unwrap();
            let _ = self.current.insert(tok);
        }
        self.current.as_ref().unwrap()
    }

    pub fn next(&mut self) -> Token {
        match self.current.take() {
            Some(t) => t,
            None => self.input.recv().unwrap(),
        }
    }

    #[instrument]
    pub fn read_form(&mut self) -> Res {
        let tok = self.next();
        event!(Level::DEBUG, "next token: {:?}", &tok);

        let val = match tok {
            Token::OpenParen => {
                let mut vals = self.read_until(&Token::CloseParen)?;
                let mut list = List::empty();
                while let Some(val) = vals.pop() {
                    list = list.cons(val);
                }
                Val::List(list)
            }
            Token::OpenBracket => {
                let vals = self.read_until(&Token::CloseBracket)?;
                Val::vec(vals)
            }
            Token::OpenBrace => {
                let map_arc = self.read_map()?;
                Val::Map(map_arc)
            }
            Token::Comment(_) => return Ok(Val::Nil), // This shouldn't happen.
            Token::Obj(obj) => read_atom(obj)?,
            Token::SingleQuote => {
                let quoted = self.read_form()?;
                Val::List(List::empty().cons(quoted).cons(Val::Symbol("quote".into())))
            }
            x => return rerr(format!("unexpected {:?}", &x)),
        };

        Ok(val)
    }

    fn read_until(&mut self, zigamorph: &Token) -> Result<Vec<Val>, MalErr> {
        let mut vals: Vec<Val> = Vec::new();

        loop {
            if self.peek() == zigamorph {
                let _ = self.next();
                return Ok(vals);
            }
            let val = self.read_form()?;
            vals.push(val);
        }
    }

    fn read_map(&mut self) -> Result<Arc<Map>, MalErr> {
        let map = Arc::new(Map::default());

        loop {
            if self.peek() == &Token::CloseBrace {
                let _ = self.next();
                return Ok(map);
            }
            let key = self.read_form()?;
            let val = self.read_form()?;
            let _ = map.insert(key, val)?;
        }
    }
}

fn read_atom(obj: String) -> Result<Val, MalErr> {
    if let Ok(i) = obj.parse::<i64>() {
        return Ok(i.into());
    } else if let Ok(x) = obj.parse::<f64>() {
        return Ok(x.into());
    }

    match obj.as_str() {
        "nil" | "Nil" | "NIL" => return Ok(Val::Nil),
        "true" | "True" | "TRUE" => return Ok(Val::True),
        "false" | "False" | "FALSE" => return Ok(Val::False),
        _ => {}
    }

    if let Some(s) = make_string(obj.as_str())? {
        let s: Arc<str> = s.into();
        Ok(Val::String(s))
    } else {
        Ok(Val::Symbol(obj.into()))
    }
}

fn make_string(chars: &str) -> Result<Option<String>, MalErr> {
    let bytes = chars.as_bytes();
    match bytes.first() {
        None => return Ok(None),
        Some(b'"') => {}
        Some(_) => return Ok(None),
    }

    if bytes.len() < 2 || bytes.last() != Some(&b'"') {
        return rerr("unbalanced string");
    }

    let sub_bytes = &bytes[1..(bytes.len() - 1)];
    let mut unescape = false;

    for w in sub_bytes.windows(2) {
        // Safety: We are looking at windows of length 2; w[0] and
        // w[1] are guaranteed to be in-bounds.
        let (a, b) = unsafe { (w.get_unchecked(0), w.get_unchecked(1)) };
        if b == &b'"' {
            if a == &b'\\' {
                unescape = true;
                break;
            } else {
                return rerr("unbalanced string");
            }
        }
    }

    if !unescape {
        //
        // TODO: Make this an unsafe conversion.
        //
        let s = String::from_utf8(sub_bytes.to_vec()).unwrap();
        return Ok(Some(s));
    }

    let matches: Vec<&[u8]> = ESCAPER.split(sub_bytes).collect();
    let v: Vec<u8> = matches.join(&b'"');
    //
    // TODO: Make this an unsafe conversion.
    //
    let s = String::from_utf8(v).unwrap();

    Ok(Some(s))
}

pub fn run() {
    use rustyline::{config::EditMode, DefaultEditor};

    let rl_conf = rustyline::Config::builder()
        .auto_add_history(true)
        .edit_mode(EditMode::Emacs)
        .build();
    let rl = DefaultEditor::with_config(rl_conf).unwrap();

    let (tx, rx) = channel::<Token>();

    let mut tokenizer = Tokenizer {
        input: rl,
        output: tx,
    };

    let mut reader = Reader {
        input: rx,
        current: None,
    };

    std::thread::spawn(move || loop {
        tokenizer.read_line();
    });

    let envt = Env::default();
    loop {
        match reader.read_form().and_then(|v| eval(&envt, v)) {
            Ok(val) => println!("{}", &val),
            Err(e) => println!("{}", &e),
        }
    }
}
