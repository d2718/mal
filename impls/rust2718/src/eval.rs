/*!
Environments and evaluation.
*/

use std::{
    collections::BTreeMap,
    sync::{Arc, RwLock},
};

use tracing::{event, Level};

use crate::{
    error,
    types::{builtin, Builtin, List, StaticFunc},
    ErrType, MalErr, Res, Val,
};

static DEFAULT_ENV: &[(&'static str, StaticFunc)] = &[("+", &builtin::add), ("-", &builtin::sub)];

#[derive(Debug)]
pub struct Env {
    map: RwLock<BTreeMap<Box<str>, Val>>,
}

impl Env {
    pub fn empty() -> Env {
        Env {
            map: RwLock::new(BTreeMap::default()),
        }
    }
}

impl Default for Env {
    fn default() -> Self {
        let mut map = BTreeMap::default();
        for (name, func) in DEFAULT_ENV.iter() {
            let f = Builtin::new(name, func);
            map.insert(name.into_boxed_str(), f.into())
        }
    }
}

pub fn eval(envt: &Env, ast: Val) -> Res {
    event!(Level::TRACE, "eval( {:?}, {:?} )", &envt, &ast);

    match ast {
        Val::List(a) => {
            if a.is_empty() {
                return Ok(ast);
            }
            let a = a.clone();
            match eval_ast(envt, a.into())? {
                Val::List(mut a) => {
                    // List shouldn't be empty;
                    let f = a.next().unwrap();
                    if let Val::Func(f) = f {
                        f.call(a)
                    } else {
                        Err(error::err(ErrType::Eval, "not callable"))
                    }
                }
                _ => unreachable!(),
            }
        }
        x => eval_ast(envt, x),
    }
}

pub fn eval_ast(envt: &Env, ast: Val) -> Res {
    event!(Level::TRACE, "eval_ast( {:?}, {:?} )", &envt, &ast);

    match ast {
        Val::Symbol(s) => match envt.map.get(s.as_ref()) {
            Some(v) => Ok(v.clone()),
            None => {
                return Err(error::err(
                    ErrType::Eval,
                    format!("undefined symbol: {}", &s),
                ))
            }
        },
        Val::List(a) => {
            let mut a = a.clone();
            let mut v: Vec<Val> = Vec::new();

            while let Some(val) = a.next() {
                v.push(eval(envt, val)?);
            }

            let mut a = List::empty();
            while let Some(val) = v.pop() {
                a = a.cons(val);
            }

            Ok(a.into())
        }
        x => Ok(x),
    }
}
