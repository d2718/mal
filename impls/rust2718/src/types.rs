/*!
Types
*/
use std::{
    convert::TryFrom,
    fmt::{Debug, Display, Formatter},
    ops::Deref,
    sync::{Arc, RwLock},
};

use ordered_float::OrderedFloat;

pub mod builtin;
mod lambda;
mod list;
mod map;
pub use lambda::{Builtin, Function, Lambda, StaticFunc};
pub use list::List;
pub use map::Map;

use crate::{error::err, ErrType, MalErr};

#[derive(Clone, Debug)]
pub enum Val {
    Nil,
    True,
    False,
    Int(i64),
    Float(OrderedFloat<f64>),
    String(Arc<str>),
    Symbol(Arc<str>),
    List(Arc<List>),
    Vector(Arc<RwLock<Vec<Val>>>),
    Map(Arc<Map>),
    Func(Arc<dyn Lambda>),
}

impl Val {
    pub fn vec<V>(v: V) -> Val
    where
        Vec<Val>: From<V>,
    {
        Val::Vector(Arc::new(RwLock::new(v.into())))
    }

    pub fn unwrap_symbol(&self) -> Result<Arc<str>, MalErr> {
        match self {
            Val::Symbol(s) => Ok(s.clone()),
            _ => MalErr::rarg("expected a symbol"),
        }
    }

    pub fn unwrap_list(&self) -> Result<Arc<List>, MalErr> {
        match self {
            Val::List(list) => Ok(list.clone()),
            Val::Nil => Ok(List::empty()),
            _ => MalErr::rarg("expected a list"),
        }
    }

    pub fn unwrap_func(&self) -> Result<Arc<dyn Lambda>, MalErr> {
        match self {
            Val::Func(f) => Ok(f.clone()),
            _ => MalErr::rarg("expected a function"),
        }
    }
}

impl Display for Val {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use Val::*;

        match self {
            Nil => write!(f, "nil"),
            True => write!(f, "true"),
            False => write!(f, "false"),
            Int(n) => write!(f, "{}", &n),
            Float(x) => write!(f, "{}", &x),
            String(ref s) => write!(f, "\"{}\"", s),
            Symbol(ref s) => write!(f, "{}", s),
            List(a) => write_list(&a, f),
            Vector(a) => write_vector(&a, f),
            Map(a) => write_map(&a, f),
            Func(fun) => write!(f, "{}", fun),
        }
    }
}

fn write_list(list: &Arc<List>, f: &mut Formatter) -> std::fmt::Result {
    match list.deref() {
        List::Nil => write!(f, "()"),
        List::Node { ref next, ref val } => {
            write!(f, "({}", val)?;
            let mut node = next.clone();
            while let List::Node { ref val, ref next } = node.deref() {
                write!(f, " {}", val)?;
                node = next.clone();
            }
            write!(f, ")")
        }
    }
}

fn write_vector(v: &Arc<RwLock<Vec<Val>>>, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "[")?;
    let handle = v.deref().read().unwrap();
    let mut val_iter = handle.iter();
    if let Some(val) = val_iter.next() {
        write!(f, "{}", val)?;
    }
    while let Some(val) = val_iter.next() {
        write!(f, " {}", val)?;
    }
    write!(f, "]")
}

fn write_map(m: &Arc<Map>, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{{")?;
    let mut val_iter = m.iter();
    if let Some((k, v)) = val_iter.next() {
        write!(f, "{} {}", k, v)?;
    }
    while let Some((k, v)) = val_iter.next() {
        write!(f, " {} {}", k, v)?;
    }
    write!(f, "}}")
}

impl From<()> for Val {
    fn from(_: ()) -> Val {
        Val::Nil
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

impl From<String> for Val {
    fn from(s: String) -> Val {
        Val::String(s.into())
    }
}

impl From<Arc<List>> for Val {
    fn from(a: Arc<List>) -> Val {
        Val::List(a.clone())
    }
}

impl From<Arc<Map>> for Val {
    fn from(a: Arc<Map>) -> Val {
        Val::Map(a.clone())
    }
}

impl From<Vec<Val>> for Val {
    fn from(v: Vec<Val>) -> Val {
        Val::Vector(Arc::new(RwLock::new(v)))
    }
}

impl From<OrderedFloat<f64>> for Val {
    fn from(x: OrderedFloat<f64>) -> Val {
        Val::Float(x)
    }
}

impl From<Builtin> for Val {
    fn from(b: Builtin) -> Val {
        Val::Func(Arc::new(b))
    }
}

impl From<Function> for Val {
    fn from(f: Function) -> Val {
        Val::Func(Arc::new(f))
    }
}

impl TryFrom<Val> for f64 {
    type Error = MalErr;

    fn try_from(v: Val) -> Result<f64, MalErr> {
        match v {
            Val::Int(n) => Ok(n as f64),
            Val::Float(x) => Ok(x.into()),
            v => Err(err(
                ErrType::Type,
                format!("{} cannot be converted to floating-point", v),
            )),
        }
    }
}

impl PartialEq for Val {
    fn eq(&self, other: &Self) -> bool {
        let b = match (self, other) {
            (Val::Nil, Val::Nil) => true,
            (Val::True, Val::True) => true,
            (Val::False, Val::False) => true,
            (Val::Int(n), Val::Int(m)) => n == m,
            (Val::Float(x), Val::Float(y)) => x == y,
            (Val::String(s), Val::String(t)) => s == t,
            (Val::Symbol(s), Val::Symbol(t)) => s == t,
            (Val::List(a), Val::List(b)) => a == b,
            (Val::Vector(u), Val::Vector(v)) => *u.read().unwrap() == *v.read().unwrap(),
            (Val::Map(m), Val::Map(n)) => m == n,
            _ => false,
        };
        b.into()
    }
}
