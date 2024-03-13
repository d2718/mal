/*!
Builtin functions.
*/
use std::sync::Arc;

use crate::{error::err, types::List, ErrType, MalErr, Res, Val};

pub fn add(args: Arc<List>) -> Res {
    let mut args = args.clone();

    let mut r: Val = 0.into();

    while let Some(v) = args.next() {
        r = match (r, v) {
            (Val::Int(n), Val::Int(m)) => Val::Int(n + m),
            (Val::Int(n), Val::Float(y)) => (y + (n as f64)).into(),
            (Val::Float(x), Val::Int(m)) => (x + (m as f64)).into(),
            (Val::Float(x), Val::Float(y)) => (x + y).into(),
            (x, y) => return Err(err(ErrType::Type, format!("cannot add {} to {}", x, y))),
        };
    }

    Ok(r)
}

pub fn sub(args: Arc<List>) -> Res {
    let mut args = args.clone();

    let mut r: Val = args
        .next()
        .ok_or_else(|| err(ErrType::Arg, "- requires at least one argument"))?;

    while let Some(v) = args.next() {
        r = match (r, v) {
            (Val::Int(n), Val::Int(m)) => Val::Int(n - m),
            (Val::Int(n), Val::Float(y)) => (-y + (n as f64)).into(),
            (Val::Float(x), Val::Int(m)) => (x - (m as f64)).into(),
            (Val::Float(x), Val::Float(y)) => (x - y).into(),
            (x, y) => return Err(err(ErrType::Type, format!("cannot add {} to {}", x, y))),
        };
    }

    Ok(r)
}
