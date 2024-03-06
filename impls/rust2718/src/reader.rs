use std::collections::BTreeMap;

use once_cell::sync::Lazy;
use ordered_float::OrderedFloat;
use regex::bytes;
use regex::Regex;

use crate::{types::Val, MalErr};

static TOKENIZER: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"[\s,]*(~@|[\[\]{}()'`~^@]|"(?:\.|[^\"])*"?|;.*|[^\s\[\]{}('",;)]*)"#)
        .expect("unable to initialize tokenizing regex")
});
static ESCAPIZER: Lazy<bytes::Regex> =
    Lazy::new(|| bytes::Regex::new(r#"\\""#).expect("unable to initialize escaping regex"));

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
                Some(s) if s.as_bytes().first() == Some(&b';') => None,
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

        let val = match tok.as_str() {
            "(" => Val::List(self.read_until(")")?),
            "[" => Val::Array(self.read_until("]")?),
            "{" => {
                let vals = self.read_until("}")?;
                let m = assemble_map(vals)?;
                Val::Map(m)
            }
            _ => Reader::read_atom(tok)?,
        };

        Ok(Some(val))
    }

    pub fn read_atom(tok: String) -> Result<Val, MalErr> {
        if let Ok(i) = tok.parse::<i64>() {
            return Ok(Val::Int(i));
        } else if let Ok(x) = tok.parse::<f64>() {
            return Ok(Val::Float(x.into()));
        }

        match tok.as_str() {
            "nil" | "Nil" | "NIL" => return Ok(Val::Nil),
            "t" | "T" => return Ok(Val::True),
            "f" | "F" => return Ok(Val::False),
            _ => {}
        }

        if let Some(s) = make_string(tok.as_str())? {
            Ok(Val::String(s))
        } else if let Some(&b':') = tok.as_bytes().get(0) {
            // If it starts with a colon, slicing the rest shoudln't panic.
            let rest = &tok[1..];
            Ok(Val::Keyword(rest.to_string()))
        } else {
            Ok(Val::Symbol(tok))
        }
    }

    pub fn read_until(&mut self, zig: &str) -> Result<Vec<Val>, MalErr> {
        let mut vals: Vec<Val> = Vec::new();

        loop {
            let val = self.read_form()?;
            let val = match val {
                None => return Err(MalErr::ReadErr("unexpected end of input".into())),
                Some(val) => val,
            };
            if let Val::Symbol(s) = &val {
                if s.as_str() == zig {
                    return Ok(vals);
                }
            }
            vals.push(val);
        }
    }
}

fn make_string(chars: &str) -> Result<Option<String>, MalErr> {
    let bytes = chars.as_bytes();
    match bytes.first() {
        None => return Ok(None),
        Some(b'"') => {}
        Some(_) => return Ok(None),
    };

    if bytes.len() < 2 {
        return Err(MalErr::ReadErr("unbalanced string".into()));
    }
    if bytes.last() != Some(&b'"') {
        return Err(MalErr::ReadErr("unbalanced string".into()));
    }
    let sub_bytes = &bytes[1..(bytes.len() - 1)];
    let mut unescape = false;

    for w in sub_bytes.windows(2) {
        let (a, b) = unsafe { (w.get_unchecked(0), w.get_unchecked(1)) };
        if b == &b'"' {
            if a == &b'\\' {
                unescape = true;
                break;
            } else {
                return Err(MalErr::ReadErr("unbalanced string".into()));
            }
        }
    }

    if !unescape {
        let s = unsafe { String::from_utf8_unchecked(sub_bytes.to_vec()) };
        return Ok(Some(s));
    }

    let matches: Vec<&[u8]> = ESCAPIZER.split(sub_bytes).collect();
    let v: Vec<u8> = matches.join(&b'"');
    let s = unsafe { String::from_utf8_unchecked(v) };

    Ok(Some(s))
}

fn assemble_map(mut vals: Vec<Val>) -> Result<BTreeMap<Val, Val>, MalErr> {
    let mut map: BTreeMap<Val, Val> = BTreeMap::new();
    let mut toks = vals.drain(..);

    loop {
        let k = match toks.next() {
            None => return Ok(map),
            Some(Val::String(s)) => Val::String(s),
            Some(Val::Keyword(s)) => Val::Keyword(s),
            v => {
                return Err(MalErr::ReadErr(
                    format!("invalid hash-map key: {:?}", &v).into(),
                ))
            }
        };
        let v = match toks.next() {
            None => {
                return Err(MalErr::ReadErr(
                    "hash-map literal must contain even number of elements".into(),
                ))
            }
            Some(v) => v,
        };
        map.insert(k, v);
    }
}
