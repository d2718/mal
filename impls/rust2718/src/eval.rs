use std::collections::{BTreeMap, HashMap};

use ordered_float::OrderedFloat;
use tracing::{event, instrument, Level};

use crate::{
    env::{EnvTree, Envr},
    hard::math,
    types::{Builtin, Fun, Lambda, TreeMap, Val, Value},
    MalErr, Res,
};

fn eval_ast_list(list: &[Value], envt: &Envr) -> Result<Vec<Value>, MalErr> {
    list.iter()
        .cloned()
        .map(|v| eval(v, envt))
        .collect::<Result<Vec<Value>, MalErr>>()
}

fn eval_ast_map(map: &TreeMap, envt: &Envr) -> Result<TreeMap, MalErr> {
    map.iter()
        .map(|(k, v)| eval(v.clone(), envt).map(|v| (k.try_clone().unwrap(), v)))
        .collect()
}

#[instrument]
pub fn eval_ast(ast: Value, envt: &Envr) -> Res {
    event!(Level::DEBUG, "({:?}, [Env])", &ast);
    match ast.as_ref() {
        Val::Symbol(s) => envt.get(s.as_ref()),
        Val::List(ref l) => Ok(Val::List(eval_ast_list(l.as_slice(), envt)?).into()),
        Val::Array(ref l) => Ok(Val::Array(eval_ast_list(l.as_slice(), envt)?).into()),
        Val::Map(ref m) => Ok(Val::Map(eval_ast_map(m, envt)?).into()),
        _ => Ok(ast),
    }
}

fn def(envt: &Envr, vals: &[Value]) -> Res {
    match vals {
        [sym, rest @ .. ] => {
            if let Val::Symbol(s) = sym.as_ref() {
                let v = eval(rest, envt)?;
                envt.set(s, v);
                return Ok(Val::Nil.into())
            }
        },
        _ => {},
    };

    return Err(MalErr::ArgErr(format!("can't define: {:?}", vals).into()))
}

fn apply(list: &[Value], envt: &Envr) -> Res {
    match list {
        [] => Ok(Val::Nil.into()),
        [first, rest @ ..] => match first.as_ref() {
            Val::Symbol(s) => match s.as_ref() {
                "def!"
            }
        }
    }
}

#[instrument]
pub fn eval(ast: Value, envt: &Envr) -> Res {
    event!(Level::DEBUG, "({:?}, [Env])", &ast);
    match ast.as_ref() {
        Val::List(ref l) if l.len() == 0 => Ok(ast),
        Val::List(ref l) => {
            let vals = eval_ast_list(l.as_slice(), envt)?;
            match vals.first().unwrap().as_ref() {
                Val::Fun(f) => f.call(&vals[1..]),
                x => Err(MalErr::ExecErr(format!("{:?} is not callable", x).into())),
            }
        }
        _ => eval_ast(ast, envt),
    }
}
