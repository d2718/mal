/*!
Environments and evaluation.
*/

use std::{ops::Deref, sync::Arc};

use tracing::{event, Level};

use crate::{
    env::Env,
    error::rerr,
    types::{Function, List, Map},
    MalErr, Res, Val,
};

pub fn eval(envt: &Arc<Env>, ast: Val) -> Res {
    event!(Level::TRACE, "eval( {:?}, {:?} )", &envt, &ast);

    let res = match ast.clone() {
        Val::List(a) => apply(envt, a),
        x => eval_ast(envt, x),
    };
    MalErr::in_form(res, ast)
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
            "do" => return do_do(envt, rest),
            "if" => return do_if(envt, rest),
            "fn" | "fn*" => return make_closure(envt, rest),
            _ => {}
        },
        _ => {}
    }

    let list = eval_ast(envt, list.into())?.unwrap_list()?;
    let func = list.car()?.unwrap_func()?;
    let rest = list.cdr()?;
    func.call(envt, rest)
}

fn define(envt: &Arc<Env>, key: Val, val: Val) -> Res {
    let key = key.unwrap_symbol()?;
    envt.set(&key, val.clone());
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
                    _ => return rerr("binding form must contain even number of elements"),
                };
                new_envt.set(&key, val);
            }
        }
        _ => return rerr("binding form must be a list or a vector"),
    }

    eval(new_envt, rest.next().unwrap_or(Val::Nil))
}

fn do_do(envt: &Arc<Env>, list: Arc<List>) -> Res {
    let mut forms = list.clone();
    while let Some(val) = forms.next() {
        if forms.is_empty() {
            return eval(envt, val);
        } else {
            let _ = eval(envt, val)?;
        }
    }
    Ok(Val::Nil)
}

fn do_if(envt: &Arc<Env>, list: Arc<List>) -> Res {
    let mut list = list.clone();
    let cond = eval(envt, list.pop()?)?;
    match cond {
        Val::Nil | Val::False => {
            let _ = list.pop()?;
            if let Some(val) = list.next() {
                eval(envt, val)
            } else {
                Ok(Val::Nil)
            }
        }
        _ => eval(envt, list.pop()?),
    }
}

fn make_closure(envt: &Arc<Env>, list: Arc<List>) -> Res {
    let mut list = list.clone();
    let mut args: Vec<Arc<str>> = Vec::new();
    match list.pop()? {
        Val::Nil => {}
        Val::Symbol(s) => args.push(s.clone()),
        Val::List(mut l) => {
            while let Some(v) = l.next() {
                args.push(v.unwrap_symbol()?.clone());
            }
        }
        Val::Vector(v) => {
            for val in v.read().unwrap().iter() {
                args.push(val.unwrap_symbol()?);
            }
        }
        _ => return rerr("expected an argument list"),
    };

    let form = list.pop()?;
    Ok(Function::define(args, envt, form).into())
}
