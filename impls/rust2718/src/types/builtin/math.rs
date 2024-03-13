/*!
Built-in arithmetic functions.
*/
use std::ops::{Add, Div, Mul, Sub};
use std::sync::Arc;

use ordered_float::OrderedFloat;

use crate::{error::err, types::List, ErrType, MalErr, Res, Val};

fn binop<F, G>(f: F, g: G, a: Val, b: Val) -> Res
where
    F: Fn(i64, i64) -> i64,
    G: Fn(f64, f64) -> f64,
{
    let v: Val = match (a, b) {
        (Val::Int(n), Val::Int(m)) => f(n, m).into(),
        (Val::Int(n), Val::Float(y)) => g(n as f64, y.into()).into(),
        (Val::Float(x), Val::Int(m)) => g(x.into(), m as f64).into(),
        (Val::Float(x), Val::Float(y)) => g(x.into(), y.into()).into(),
        (x, y) => {
            return Err(err(
                ErrType::Type,
                format!("attempt to perform arithmetic with {} & {}", x, y),
            ))
        }
    };

    Ok(v)
}

pub fn add(args: Arc<List>) -> Res {
    let mut args = args.clone();
    let mut r: Val = 0.into();
    while let Some(v) = args.next() {
        r = binop(Add::add, Add::add, r, v)?;
    }
    Ok(r)
}

pub fn mul(args: Arc<List>) -> Res {
    let mut args = args.clone();
    let mut r: Val = 1.into();
    while let Some(v) = args.next() {
        r = binop(Mul::mul, Mul::mul, r, v)?;
    }
    Ok(r)
}

pub fn sub(args: Arc<List>) -> Res {
    let mut args = args.clone();

    let mut r: Val = args
        .next()
        .ok_or_else(|| err(ErrType::Arg, "- requires at least one argument"))?;

    while let Some(v) = args.next() {
        r = binop(Sub::sub, Sub::sub, r, v)?;
    }

    Ok(r)
}

pub fn div(args: Arc<List>) -> Res {
    let mut args = args.clone();
    let num = args
        .next()
        .ok_or_else(|| err(ErrType::Arg, "/ requires at least one argument"))?;
    let den = match args.next() {
        None => return Ok(num),
        Some(v) => v,
    };

    let v: Val = match (num, den) {
        (_, Val::Int(0)) | (_, Val::Float(OrderedFloat(0.0))) => {
            return Err(err(ErrType::Arg, "division by zero"))
        }
        (Val::Int(n), Val::Int(m)) if n % m == 0 => (n / m).into(),
        (Val::Int(n), Val::Int(m)) => (n as f64 / m as f64).into(),
        (x, y) => binop(Div::div, Div::div, x, y)?,
    };

    Ok(v)
}
