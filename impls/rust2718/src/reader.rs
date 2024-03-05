use once_cell::sync::Lazy;
use regex::Regex;

use crate::{types::Val, MalErr};

static TOKENIZER: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"[\s,]*(~@|[\[\]{}()'`~^@]|"(?:\.|[^\"])*"?|;.*|[^\s\[\]{}('",;)]*)"#)
        .expect("unable to initialize tokenizing regex")
});

pub struct Reader {
    pos: usize,
    tokens: Vec<String>,
}

impl Reader {
    pub fn tokenize(text: &str) -> Reader {
        let tokens: Vec<String> = TOKENIZER
            .captures_iter(text)
            .filter_map(|c| match c.get(1).map(|m| m.as_str().trim()) {
                None | Some("") => None,
                Some(s) => Some(s.to_string()),
            })
            .collect();

        Reader { pos: 0, tokens }
    }

    pub fn peek(&self) -> Option<String> {
        self.tokens.get(self.pos).cloned()
    }

    pub fn next(&mut self) -> Option<String> {
        let tok = self.tokens.get(self.pos).cloned();
        if self.pos < self.tokens.len() {
            self.pos += 1;
        }
        tok
    }

    pub fn read_form(&mut self) -> Result<Option<Val>, MalErr> {
        let tok = match self.next() {
            Some(tok) => tok,
            None => return Ok(None),
        };

        if tok.as_str() == "(" {
            let val = self.read_list()?;
            Ok(Some(val))
        } else {
            let val = Reader::read_atom(tok)?;
            Ok(Some(val))
        }
    }

    pub fn read_atom(tok: String) -> Result<Val, MalErr> {
        if let Ok(i) = tok.parse::<i64>() {
            Ok(Val::Int(i))
        } else if let Ok(x) = tok.parse::<f64>() {
            Ok(Val::Float(x))
        } else {
            Ok(Val::Symbol(tok))
        }
    }

    pub fn read_list(&mut self) -> Result<Val, MalErr> {
        let mut vals: Vec<Val> = Vec::new();

        loop {
            let val = self.read_form()?;
            let val = match val {
                None => return Err(MalErr::ReadErr("unexpected end of input".into())),
                Some(val) => val,
            };
            if let Val::Symbol(s) = &val {
                if s.as_str() == ")" {
                    return Ok(Val::List(vals));
                }
            }
            vals.push(val)
        }
    }

    pub fn read_array(&mut self) -> Result<Val, MalErr> {
        let mut vals: Vec<Val> = Vec::new();

        loop {
            let val = self.read_form()?;
            let val = match val {
                None => return Err(MalErr::ReadErr("unexpected end of input".into())),
                Some(val) => val,
            };
            if let Val::Symbol(s) = &val {
                if s.as_str() == "]" {
                    return Ok(Val::Array(vals));
                }
            }
            vals.push(val)
        }
    }
}
