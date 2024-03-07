use std::collections::{BTreeMap, HashMap};

use ordered_float::OrderedFloat;
use tracing::{event, instrument, Level};

use crate::{
    hard::math,
    types::{Fun, Lambda, Val},
    MalErr,
};

pub type Env = HashMap<String, Fun>;

const DEFAULT_ENV: &[(&str, &dyn Lambda)] = &[
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
        .map(|(sym, f)| (sym.to_string(), Fun::new(sym.to_string(), Box::new(f))))
        .collect()
}

fn eval_ast_list(list: &[Val], envt: &Env) -> Result<Vec<Val>, MalErr> {
    list.iter()
        .cloned()
        .map(|v| eval(v, envt))
        .collect::<Result<Vec<Val>, MalErr>>()
}

fn eval_ast_map(map: &BTreeMap<Val, Val>, envt: &Env) -> Result<BTreeMap<Val, Val>, MalErr> {
    map.iter()
        .map(|(k, v)| eval(v.clone(), envt).map(|v| (k.clone(), v)))
        .collect()
}

#[instrument]
pub fn eval_ast(ast: Val, envt: &Env) -> Result<Val, MalErr> {
    event!(Level::DEBUG, "({:?}, [Env])", &ast);
    match ast {
        Val::Symbol(ref s) => match envt.get(s) {
            Some(f) => Ok(Val::Fun(f.clone())),
            None => Err(MalErr::ExecErr(format!("unresolved symbol: {}", &s).into())),
        },
        Val::List(ref l) => Ok(Val::List(eval_ast_list(l.as_slice(), envt)?)),
        Val::Array(ref l) => Ok(Val::Array(eval_ast_list(l.as_slice(), envt)?)),
        Val::Map(ref m) => Ok(Val::Map(eval_ast_map(m, envt)?)),
        x => Ok(x),
    }
}

#[instrument]
pub fn eval(ast: Val, envt: &Env) -> Result<Val, MalErr> {
    event!(Level::DEBUG, "({:?}, [Env])", &ast);
    match ast {
        Val::List(ref l) if l.len() == 0 => Ok(ast),
        Val::List(ref l) => {
            let vals = eval_ast_list(l.as_slice(), envt)?;
            if let Some(Val::Fun(f)) = vals.first() {
                f.call(&vals[1..])
            } else if let Some(x) = vals.first() {
                Err(MalErr::ExecErr(format!("{:?} is not callable", x).into()))
            } else {
                unreachable!()
            }
        }
        v => eval_ast(v, envt),
    }
}
