/*!
Environments and evaluation.
*/

use std::{
    collections::BTreeMap,
    ops::Deref,
    sync::{Arc, RwLock},
};

use tracing::{event, Level};

use crate::{
    env::Env,
    error,
    types::{builtin::math, Builtin, Lambda, List, Map, StaticFunc},
    ErrType, MalErr, Res, Val,
};

pub fn eval(envt: &Arc<Env>, ast: Val) -> Res {
    event!(Level::TRACE, "eval( {:?}, {:?} )", &envt, &ast);

    match ast {
        Val::List(a) => apply(envt, a),
        x => eval_ast(envt, x),
    }
}

pub fn eval_ast(envt: &Arc<Env>, ast: Val) -> Res {
    event!(Level::TRACE, "eval_ast( {:?}, {:?} )", &envt, &ast);

    match ast {
        Val::Symbol(s) => envt.get(s.as_ref()).map(|v| v.clone()),
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
        Val::Vector(a) => {
            let v: Vec<Val> = a
                .read()
                .unwrap()
                .iter()
                .cloned()
                .map(|v| eval(envt, v))
                .collect::<Result<Vec<_>, MalErr>>()?;
            Ok(v.into())
        }
        Val::Map(a) => {
            let new_map = Arc::new(Map::default());
            for (k, v) in a.iter() {
                new_map.insert(k, eval(envt, v)?)?;
            }
            let v: Val = new_map.into();
            Ok(v)
        }
        x => Ok(x),
    }
}

fn apply(envt: &Arc<Env>, list: Arc<List>) -> Res {
    event!(Level::TRACE, "apply([ Env ], {:?})", &list);

    let car = match list.car() {
        Ok(val) => val,
        Err(_) => return Ok(list.into()),
    };
    let rest = list.cdr()?;

    match car {
        Val::Symbol(s) => match s.deref() {
            "def!" => return define(envt, rest.car()?, eval(envt, rest.cdr()?.car()?)?),
            "let" | "let*" => return do_let(&Env::child_of(envt), rest),
            _ => {}
        },
        _ => {}
    }

    let list = eval_ast(envt, list.into())?.unwrap_list()?;
    let func = list.car()?.unwrap_func()?;
    let rest = list.cdr()?;
    func.call(rest)
}

fn define(envt: &Arc<Env>, key: Val, val: Val) -> Res {
    envt.set(&key.unwrap_symbol()?, val.clone());
    Ok(val)
}

fn do_let(new_envt: &Arc<Env>, rest: Arc<List>) -> Res {
    let mut rest = rest.clone();
    match rest.pop()? {
        Val::List(mut a) => loop {
            let key = match a.next() {
                Some(s) => s.unwrap_symbol()?,
                None => break,
            };
            let val = eval(new_envt, a.pop()?)?;
            new_envt.set(&key, val);
        },
        Val::Vector(a) => {
            for chunk in a.read().unwrap().chunks(2) {
                let (key, val) = match chunk {
                    [k, v] => (k.unwrap_symbol()?, eval(new_envt, v.clone())?),
                    _ => return MalErr::rarg("binding for must contain even number of elements"),
                };
                new_envt.set(&key, val);
            }
        }
        _ => return MalErr::rarg("binding form must be a list or a vector"),
    }

    eval(new_envt, rest.next().unwrap_or(Val::Nil))
}
