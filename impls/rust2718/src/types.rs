/*!
Types.
*/
mod lambda;
pub use lambda::{Fun, Lambda};

use std::{
    collections::BTreeMap,
    fmt::{Debug, Display, Formatter, Write},
};

use ordered_float::OrderedFloat;

use crate::MalErr;

#[derive(Clone, PartialEq, PartialOrd, Ord, Eq, Hash)]
pub enum Val {
    Nil,
    True,
    False,
    Symbol(String),
    Keyword(String),
    String(String),
    Float(OrderedFloat<f64>),
    Int(i64),
    List(Vec<Val>),
    Array(Vec<Val>),
    Map(BTreeMap<Val, Val>),
    Fun(Fun),
}

fn write_collection(
    f: &mut Formatter<'_>,
    vals: &[Val],
    open: &str,
    close: &str,
) -> std::fmt::Result {
    write!(f, "{}", open)?;
    let mut stuff = vals.iter();
    if let Some(ref v) = stuff.next() {
        write!(f, "{}", v)?;
    }
    for v in stuff {
        write!(f, " {}", v)?;
    }
    write!(f, "{}", close)
}

fn write_map(f: &mut Formatter<'_>, m: &BTreeMap<Val, Val>) -> std::fmt::Result {
    write!(f, "{{")?;
    let mut stuff = m.iter();
    if let Some((k, v)) = stuff.next() {
        write!(f, "{} {}", k, v)?;
    }
    for (k, v) in stuff {
        write!(f, " {} {}", k, v)?
    }
    write!(f, "}}")
}

impl Display for Val {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Val::Nil => write!(f, "nil"),
            Val::True => write!(f, "T"),
            Val::False => write!(f, "F"),
            Val::String(ref s) => Display::fmt(s, f),
            Val::Keyword(ref s) => write!(f, ":{}", s),
            Val::Symbol(ref s) => Display::fmt(s, f),
            Val::Float(ref x) => Display::fmt(x, f),
            Val::Int(ref i) => Display::fmt(i, f),
            Val::List(v) => write_collection(f, v.as_slice(), "(", ")"),
            Val::Array(v) => write_collection(f, v.as_slice(), "[", "]"),
            Val::Map(m) => write_map(f, m),
            Val::Fun(_) => Display::fmt(self, f),
        }
    }
}

fn debug_write_collection(
    f: &mut Formatter<'_>,
    vals: &[Val],
    open: &str,
    close: &str,
) -> std::fmt::Result {
    write!(f, "{}", open)?;
    let mut stuff = vals.iter();
    if let Some(ref v) = stuff.next() {
        write!(f, "{:?}", v)?;
    }
    for v in stuff {
        write!(f, " {:?}", v)?;
    }
    write!(f, "{}", close)
}

fn debug_write_map(f: &mut Formatter<'_>, m: &BTreeMap<Val, Val>) -> std::fmt::Result {
    write!(f, "{{")?;
    let mut stuff = m.iter();
    if let Some((k, v)) = stuff.next() {
        write!(f, "{:?} {:?}", k, v)?;
    }
    for (k, v) in stuff {
        write!(f, " {:?} {:?}", k, v)?
    }
    write!(f, "}}")
}

impl Debug for Val {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Val::Nil
            | Val::True
            | Val::False
            | Val::Keyword(_)
            | Val::Symbol(_)
            | Val::Float(_)
            | Val::Int(_) => Display::fmt(self, f),
            Val::String(ref s) => write!(f, "{:?}", s),
            Val::List(v) => debug_write_collection(f, v.as_slice(), "(", ")"),
            Val::Array(v) => debug_write_collection(f, v.as_slice(), "[", "]"),
            Val::Map(m) => debug_write_map(f, m),
            Val::Fun(_) => Debug::fmt(self, f),
        }
    }
}

impl From<bool> for Val {
    fn from(b: bool) -> Val {
        if b {
            Val::True
        } else {
            Val::False
        }
    }
}

impl From<i64> for Val {
    fn from(n: i64) -> Val {
        Val::Int(n)
    }
}

impl From<f64> for Val {
    fn from(x: f64) -> Val {
        Val::Float(OrderedFloat(x))
    }
}

impl From<OrderedFloat<f64>> for Val {
    fn from(x: OrderedFloat<f64>) -> Val {
        Val::Float(x)
    }
}
