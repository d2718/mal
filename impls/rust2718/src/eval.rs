use std::collections::{BTreeMap, HashMap};

use ordered_float::OrderedFloat;
use tracing::{event, instrument, Level};

use crate::{
    hard::math,
    types::{Fun, Lambda, TreeMap, Val, Value},
    MalErr, Res,
};

pub type Env = HashMap<Box<str>, Fun>;

const DEFAULT_ENV: &[(&str, &Lambda)] = &[
    ("+", &math::add),
    ("-", &math::sub),
    ("*", &math::mul),
    ("/", &math::div),
    ("div", &math::idiv),
    ("rem", &math::rem),
];

pub fn default_env() -> Env {
    DEFAULT_ENV
        .iter()
        .copied()
        .map(|(sym, f)| {
            let b: Box<str> = sym.into();
            (b, Fun::new(sym.to_string(), f))
        })
        .collect()
}

fn eval_ast_list(list: &[Value], envt: &Env) -> Result<Vec<Value>, MalErr> {
    list.iter()
        .cloned()
        .map(|v| eval(v, envt))
        .collect::<Result<Vec<Value>, MalErr>>()
}

fn eval_ast_map(map: &TreeMap, envt: &Env) -> Result<TreeMap, MalErr> {
    map.iter()
        .map(|(k, v)| eval(v.clone(), envt).map(|v| (k.try_clone().unwrap(), v)))
        .collect()
}

#[instrument]
pub fn eval_ast(ast: Value, envt: &Env) -> Res {
    event!(Level::DEBUG, "({:?}, [Env])", &ast);
    match *ast {
        Val::Symbol(ref s) => match envt.get(s).as_deref() {
            Some(f) => Ok(ast.clone()),
            None => Err(MalErr::ExecErr(format!("unresolved symbol: {}", &s).into())),
        },
        Val::List(ref l) => Ok(Val::List(eval_ast_list(l.as_slice(), envt)?).into()),
        Val::Array(ref l) => Ok(Val::Array(eval_ast_list(l.as_slice(), envt)?).into()),
        Val::Map(ref m) => Ok(Val::Map(eval_ast_map(m, envt)?).into()),
        x => Ok(ast),
    }
}

#[instrument]
pub fn eval(ast: Value, envt: &Env) -> Res {
    event!(Level::DEBUG, "({:?}, [Env])", &ast);
    match *ast {
        Val::List(ref l) if l.len() == 0 => Ok(ast),
        Val::List(ref l) => {
            let vals = eval_ast_list(l.as_slice(), envt)?;
            match **(vals.first().unwrap()) {
                Val::Fun(f) => f.call(&vals[1..]),
                x => Err(MalErr::ExecErr(format!("{:?} is not callable", x).into())),
            }
        }
        v => eval_ast(ast, envt),
    }
}
