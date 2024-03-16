/*!
Built-in arithmetic functions.
*/
use std::sync::Arc;
use std::{
    cmp::Ordering,
    ops::{Add, Div, Mul, Sub},
};

use ordered_float::OrderedFloat;

use crate::{
    error::err,
    types::{List, StaticFunc},
    ErrType, MalErr, Res, Val,
};

pub const BUILTINS: &[(&str, &StaticFunc)] = &[
    ("+", &add),
    ("-", &sub),
    ("*", &mul),
    ("/", &div),
    ("div", &int_div),
    ("mod", &int_mod),
    ("sqrt", &sqrt),
    ("<", &less_than),
    ("<=", &less_or_eq),
    (">", &greater_than),
    (">=", &greater_or_eq),
];

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

fn bincmp(name: &str, args: &Arc<List>, ok: &[Ordering]) -> Res {
    let mut args = args.clone();
    let (a, b) = match (args.next(), args.next()) {
        (Some(a), Some(b)) => (a, b),
        _ => {
            return Err(err(
                ErrType::Arg,
                format!("{} requires two arguments", name),
            ))
        }
    };

    let (x, y): (f64, f64) = match (a.clone().try_into(), b.clone().try_into()) {
        (Ok(x), Ok(y)) => (x, y),
        _ => {
            return Err(err(
                ErrType::Type,
                format!("cannot compare {} with {}", &a, &b),
            ))
        }
    };

    let b = match x.partial_cmp(&y) {
        Some(b) => b,
        None => return Ok(false.into()),
    };
    Ok(ok.contains(&b).into())
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

pub fn int_div(args: Arc<List>) -> Res {
    let mut args = args.clone();
    let dividend = args
        .next()
        .ok_or_else(|| MalErr::arg(format!("div requires two arguments")))?;
    let divisor = args
        .next()
        .ok_or_else(|| MalErr::arg(format!("div requires two arguments")))?;

    match (dividend, divisor) {
        (_, Val::Int(0)) => Err(MalErr::arg("division by zero")),
        (Val::Int(n), Val::Int(m)) => Ok((n / m).into()),
        _ => Err(MalErr::arg("div requires integer arguments")),
    }
}

pub fn int_mod(args: Arc<List>) -> Res {
    let mut args = args.clone();
    let dividend = args
        .next()
        .ok_or_else(|| MalErr::arg("mod requires two arguments"))?;
    let divisor = args
        .next()
        .ok_or_else(|| MalErr::arg("mod requires two arguments"))?;

    match (dividend, divisor) {
        (_, Val::Int(0)) => Err(MalErr::arg("division by zero")),
        (Val::Int(n), Val::Int(m)) => Ok((n % m).into()),
        _ => Err(MalErr::arg("mod requires integer arguments")),
    }
}

pub fn sqrt(args: Arc<List>) -> Res {
    let arg = args.car()?;

    let arg: f64 = match arg {
        Val::Int(n) => n as f64,
        Val::Float(x) => x.into(),
        _ => MalErr::rarg("sqrt requires numeric argument")?,
    };

    if arg < 0.0 {
        MalErr::rarg("sqrt requres non-negative argument")?;
    }

    Ok(arg.sqrt().into())
}

pub fn less_than(args: Arc<List>) -> Res {
    bincmp("<", &args, &[Ordering::Less])
}
pub fn less_or_eq(args: Arc<List>) -> Res {
    bincmp("<=", &args, &[Ordering::Less, Ordering::Equal])
}
pub fn greater_than(args: Arc<List>) -> Res {
    bincmp(">", &args, &[Ordering::Greater])
}
pub fn greater_or_eq(args: Arc<List>) -> Res {
    bincmp(">=", &args, &[Ordering::Greater, Ordering::Equal])
}
