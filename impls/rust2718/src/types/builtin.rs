/*!
Builtin functions.
*/
use std::sync::Arc;

use crate::{
    error::err,
    types::{List, StaticFunc},
    ErrType, MalErr, Res, Val,
};

pub mod math;

pub const BUILTINS: &[(&str, &StaticFunc)] = &[
    ("list?", &list_p),
    ("count", &count),
    ("empty?", &is_empty),
    ("list", &make_list),
    ("=", &equal),
    ("prn", &prn),
];

pub fn list_p(args: Arc<List>) -> Res {
    match args.car() {
        Ok(Val::List(_)) => Ok(Val::True),
        _ => Ok(Val::False),
    }
}

pub fn count(args: Arc<List>) -> Res {
    match args.car() {
        Ok(Val::List(list)) => Ok(list.len().into()),
        Ok(Val::Nil) => Ok(0.into()),
        _ => Err(err(ErrType::Type, "count requires a countable argument")),
    }
}

pub fn is_empty(args: Arc<List>) -> Res {
    Ok((count(args)? == Val::Int(0)).into())
}

pub fn make_list(args: Arc<List>) -> Res {
    Ok(args.into())
}

pub fn equal(args: Arc<List>) -> Res {
    let mut args = args.clone();
    let (a, b) = match (args.next(), args.next()) {
        (Some(a), Some(b)) => (a, b),
        _ => return MalErr::rarg("= requires two arguments"),
    };

    Ok((a == b).into())
}

pub fn prn(args: Arc<List>) -> Res {
    let mut args = args.clone();
    while let Some(a) = args.next() {
        print!(" {}", a);
    }
    println!("");
    Ok(Val::Nil)
}
