/*!
Intrinsic arithmetic functions.
*/
use std::{env::temp_dir, sync::Arc};

use ordered_float::OrderedFloat;

use crate::{
    types::{TreeMap, Val, Value},
    MalErr, Res,
};

pub fn add(args: &[Value]) -> Res {
    let mut r = Val::Int(0);

    for v in args.iter() {
        r = match (r, **v) {
            (Val::Int(n), Val::Int(m)) => Val::Int(n + m),
            (Val::Int(n), Val::Float(x)) | (Val::Float(x), Val::Int(n)) => {
                let n = n.clone() as f64;
                Val::Float(x + n)
            }
            (Val::Float(x), Val::Float(y)) => Val::Float((x + y).into()),
            (_, v) => return Err(MalErr::ArgErr(format!("invalid argument: {:?}", v).into())),
        };
    }

    Ok(r.into())
}

pub fn mul(args: &[Value]) -> Res {
    let mut r = Val::Int(1);

    for v in args.iter() {
        r = match (r, **v) {
            (Val::Int(n), Val::Int(m)) => Val::Int(n * m),
            (Val::Int(n), Val::Float(x)) | (Val::Float(x), Val::Int(n)) => {
                let n = n.clone() as f64;
                Val::Float(x + n)
            }
            (Val::Float(x), Val::Float(y)) => Val::Float((x * y).into()),
            (_, v) => return Err(MalErr::ArgErr(format!("invalid argument: {:?}", v).into())),
        };
    }

    Ok(r.into())
}

pub fn sub(args: &[Value]) -> Res {
    let mut args = args.iter();
    let temp = args
        .next()
        .ok_or_else(|| MalErr::ArgErr("- requires at least ome argument".into()))?;
    let mut r = (*temp)
        .try_clone()
        .map_err(|_| MalErr::ArgErr("- requires numerical arguments".into()))?;

    for v in args {
        r = match (r, **v) {
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

    Ok(r.into())
}

pub fn div(args: &[Value]) -> Res {
    let mut args = args.iter();
    let temp = args
        .next()
        .ok_or_else(|| MalErr::ArgErr("div requires two arguments".into()))?;
    let mut r = (*temp)
        .try_clone()
        .map_err(|_| MalErr::ArgErr("div requires numerical arguments".into()))?;

    for v in args {
        if **v == Val::Int(0) || **v == Val::Float(OrderedFloat(0.0)) {
            return Err(MalErr::ArgErr("division by zero".into()));
        }
        r = match (r, **v) {
            (Val::Int(n), Val::Int(m)) if n % m == 0 => Val::Int(n / m),
            (Val::Int(n), Val::Int(m)) => Val::Float(((n as f64) / (m as f64)).into()),
            (Val::Int(n), Val::Float(x)) => Val::Float(((n as f64) / x.into_inner()).into()),
            (Val::Float(x), Val::Int(n)) => Val::Float((x / n as f64).into()),
            (Val::Float(x), Val::Float(y)) => Val::Float((x / y).into()),
            (_, v) => return Err(MalErr::ArgErr(format!("invalid argument: {:?}", v).into())),
        };
    }

    Ok(r.into())
}

pub fn idiv(args: &[Value]) -> Res {
    match args {
        [a, b] => match (a.as_ref(), b.as_ref()) {
            (Val::Int(_), Val::Int(0)) => return Err(MalErr::ArgErr("division by zero".into())),
            (Val::Int(n), Val::Int(m)) => return Ok(Val::Int(n / m).into()),
            _ => {}
        },
        _ => {}
    }
    Err(MalErr::ArgErr(
        "div requires exactly two integer arguments".into(),
    ))
}

pub fn rem(args: &[Value]) -> Res {
    match args {
        [a, b] => match (a.as_ref(), b.as_ref()) {
            (Val::Int(_), Val::Int(0)) => return Err(MalErr::ArgErr("division by zero".into())),
            (Val::Int(n), Val::Int(m)) => return Ok(Val::Int(n % m).into()),
            _ => {}
        },
        _ => {}
    }
    Err(MalErr::ArgErr(
        "div requires exactly two integer arguments".into(),
    ))
}
