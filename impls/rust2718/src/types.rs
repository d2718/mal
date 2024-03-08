/*!
Types.
*/
mod lambda;
pub use lambda::{Fun, Lambda};

use std::{
    collections::BTreeMap,
    fmt::{Debug, Display, Formatter},
    sync::Arc,
};

use ordered_float::OrderedFloat;

use crate::MalErr;

pub type TreeMap = BTreeMap<Val, Value>;

#[derive(PartialEq, PartialOrd, Ord, Eq, Hash)]
pub enum Val {
    Nil,
    True,
    False,
    Symbol(Box<str>),
    Keyword(Box<str>),
    String(Box<str>),
    Float(OrderedFloat<f64>),
    Int(i64),
    List(Vec<Value>),
    Array(Vec<Value>),
    Map(TreeMap),
    Fun(Fun),
}

impl Val {
    pub fn try_clone(&self) -> Result<Val, MalErr> {
        let val = match self {
            Val::Nil => Val::Nil,
            Val::True => Val::True,
            Val::False => Val::False,
            Val::Symbol(b) => Val::Symbol(b.clone()),
            Val::Keyword(b) => Val::Keyword(b.clone()),
            Val::String(b) => Val::String(b.clone()),
            Val::Float(x) => Val::Float(*x),
            Val::Int(n) => Val::Int(*n),
            Val::List(v) => Val::List(v.clone()),
            Val::Array(v) => Val::Array(v.clone()),
            _ => {
                return Err(MalErr::TypeErr(std::borrow::Cow::from(format!(
                    "not clonable: {:?}",
                    self
                ))))
            }
        };
        Ok(val)
    }
}

pub type Value = Arc<Val>;

fn write_collection(
    f: &mut Formatter<'_>,
    vals: &[Value],
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

fn write_map(f: &mut Formatter<'_>, m: &TreeMap) -> std::fmt::Result {
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
    vals: &[Value],
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

fn debug_write_map(f: &mut Formatter<'_>, m: &TreeMap) -> std::fmt::Result {
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

pub trait IntoValue {
    fn into(self) -> Value;
}

impl IntoValue for bool {
    fn into(self) -> Value {
        if self {
            Arc::new(Val::True)
        } else {
            Arc::new(Val::False)
        }
    }
}

impl IntoValue for i64 {
    fn into(self) -> Value {
        Arc::new(Val::Int(self))
    }
}

impl IntoValue for f64 {
    fn into(self) -> Value {
        Arc::new(Val::Float(OrderedFloat(self)))
    }
}

impl IntoValue for OrderedFloat<f64> {
    fn into(self) -> Value {
        Arc::new(Val::Float(self))
    }
}

impl IntoValue for String {
    fn into(self) -> Value {
        Arc::new(Val::String(self.into_boxed_str()))
    }
}

impl IntoValue for &str {
    fn into(self) -> Value {
        let b: Box<str> = Into::into(self);
        Arc::new(Val::String(b))
    }
}

impl IntoValue for Fun {
    fn into(self) -> Value {
        Arc::new(Val::Fun(self))
    }
}
