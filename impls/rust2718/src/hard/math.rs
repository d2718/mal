/*!
Intrinsic arithmetic functions.
*/
use ordered_float::OrderedFloat;

use crate::{types::Val, MalErr, Res};

pub fn add(args: &[Val]) -> Res {
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

pub fn mul(args: &[Val]) -> Res {
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

pub fn sub(args: &[Val]) -> Res {
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

pub fn div(args: &[Val]) -> Res {
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

pub fn idiv(args: &[Val]) -> Res {
    match args {
        [Val::Int(_), Val::Int(0)] => Err(MalErr::ArgErr("division by zero".into())),
        [Val::Int(n), Val::Int(m)] => Ok(Val::Int(n / m)),
        _ => {
            return Err(MalErr::ArgErr(
                "div requires exactly two integer arguments".into(),
            ))
        }
    }
}

pub fn rem(args: &[Val]) -> Res {
    match args {
        [Val::Int(_), Val::Int(0)] => Err(MalErr::ArgErr("division by zero".into())),
        [Val::Int(n), Val::Int(m)] => Ok(Val::Int(n % m)),
        _ => {
            return Err(MalErr::ArgErr(
                "rem requires exactly two integer arguments".into(),
            ))
        }
    }
}
