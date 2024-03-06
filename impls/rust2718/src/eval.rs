use std::collections::HashMap;

use ordered_float::OrderedFloat;

use crate::{types::Val, MalErr};

pub type Env = HashMap<String, Box<&'static dyn Lambda>>;

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
        r = match (r, &v) {
            (Val::Int(0), Val::Float(x)) | (_, Val::Float(x)) if x == &0.0 => {
                return Err(MalErr::ArgErr("division by zero".into()))
            }
            (Val::Int(n), Val::Int(m)) if n % m == 0 => Val::Int(n / m),
            (Val::Int(n), Val::Float(x)) => {
                let n = OrderedFloat(n as f64);
                Val::Float(n / x)
            }
            (Val::Float(x), Val::Int(n)) => Val::Float(x / (n.clone() as f64)),
            (Val::Float(x), Val::Float(y)) => Val::Float(x / y),
            (_, v) => return Err(MalErr::ArgErr(format!("invalid argument: {:?}", v).into())),
        };
    }

    Ok(r)
}

const DEFAULT_ENV: &[(&str, &dyn Lambda)] = &[("+", &add), ("-", &sub), ("*", &mul), ("/", &div)];

pub fn default_env() -> Env {
    DEFAULT_ENV
        .iter()
        .map(|(sym, f)| (sym.to_string(), Box::new(*f)))
        .collect()
}

pub fn eval_ast(ast: Val, envt: &Env) -> Result<Val, MalErr> {
    match ast {
        Val::Symbol(ref s) -> 
    }

    
    todo!()
}
