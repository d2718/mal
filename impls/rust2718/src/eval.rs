use std::collections::HashMap;

use ordered_float::OrderedFloat;
use tracing::{event, instrument, Level};

use crate::{
    types::{Fun, Lambda, Val},
    MalErr,
};

pub type Env = HashMap<String, Fun>;

fn add(args: &[Val]) -> Result<Val, MalErr> {
    let mut r = Val::Int(0);

    for v in args.iter() {
        r = match (&r, v) {
            (Val::Int(n), Val::Int(m)) => Val::Int(n + m),
            (Val::Int(n), Val::Float(x)) | (Val::Float(x), Val::Int(n)) => {
                let n = n.clone() as f64;
                Val::Float(x + n)
            }
            (Val::Float(x), Val::Float(y)) => Val::Float(*x + *y),
            (_, v) => return Err(MalErr::ArgErr(format!("invalid argument: {:?}", v).into())),
        };
    }

    Ok(r)
}

fn mul(args: &[Val]) -> Result<Val, MalErr> {
    let mut r = Val::Int(1);

    for v in args.iter() {
        r = match (&r, v) {
            (Val::Int(n), Val::Int(m)) => Val::Int(n * m),
            (Val::Int(n), Val::Float(x)) | (Val::Float(x), Val::Int(n)) => {
                let n = n.clone() as f64;
                Val::Float(x + n)
            }
            (Val::Float(x), Val::Float(y)) => Val::Float(*x * *y),
            (_, v) => return Err(MalErr::ArgErr(format!("invalid argument: {:?}", v).into())),
        };
    }

    Ok(r)
}

fn sub(args: &[Val]) -> Result<Val, MalErr> {
    let mut args = args.iter();
    let mut r = args
        .next()
        .ok_or_else(|| MalErr::ArgErr("- requires at least ome argument".into()))?
        .clone();

    for v in args {
        r = match (r, v) {
            (Val::Int(n), Val::Int(m)) => Val::Int(n - m),
            (Val::Int(n), Val::Float(x)) => {
                let n = n as f64;
                Val::Float(-x + n)
            }
            (Val::Float(x), Val::Int(n)) => {
                let n = n.clone() as f64;
                Val::Float(x - n)
            }
            (Val::Float(x), Val::Float(y)) => Val::Float(x - y),
            (_, v) => return Err(MalErr::ArgErr(format!("invalid argument: {:?}", v).into())),
        };
    }

    Ok(r)
}

fn div(args: &[Val]) -> Result<Val, MalErr> {
    let mut args = args.iter();
    let mut r = args
        .next()
        .ok_or_else(|| MalErr::ArgErr("/ requires at least ome argument".into()))?
        .clone();

    for v in args {
        if *v == Val::Int(0) || *v == Val::Float(OrderedFloat(0.0)) {
            return Err(MalErr::ArgErr("division by zero".into()));
        }
        r = match (r, &v) {
            (Val::Int(n), Val::Int(m)) if n % m == 0 => (n / m).into(),
            (Val::Int(n), Val::Int(m)) => ((n as f64) / (*m as f64)).into(),
            (Val::Int(n), Val::Float(x)) => ((n as f64) / x.into_inner()).into(),
            (Val::Float(x), Val::Int(n)) => (x / (*n) as f64).into(),
            (Val::Float(x), Val::Float(y)) => (x / y).into(),
            (_, v) => return Err(MalErr::ArgErr(format!("invalid argument: {:?}", v).into())),
        };
    }

    Ok(r)
}

const DEFAULT_ENV: &[(&str, &dyn Lambda)] = &[("+", &add), ("-", &sub), ("*", &mul), ("/", &div)];

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

#[instrument]
pub fn eval_ast(ast: Val, envt: &Env) -> Result<Val, MalErr> {
    event!(Level::DEBUG, "({:?}, [Env])", &ast);
    match ast {
        Val::Symbol(ref s) => match envt.get(s) {
            Some(f) => Ok(Val::Fun(f.clone())),
            None => Err(MalErr::ExecErr(format!("unresolved symbol: {}", &s).into())),
        },
        Val::List(ref l) => Ok(Val::List(eval_ast_list(l.as_slice(), envt)?)),
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
