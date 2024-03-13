/*!
Types
*/
use std::{
    fmt::{Debug, Display, Formatter},
    ops::Deref,
    sync::{Arc, RwLock},
};

use ordered_float::OrderedFloat;

pub mod builtin;
mod lambda;
mod list;
pub use lambda::{Builtin, Lambda, StaticFunc};
pub use list::List;

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
    Func(Arc<dyn Lambda>),
}

impl Val {
    pub fn vec<V>(v: V) -> Val
    where
        Vec<Val>: From<V>,
    {
        Val::Vector(Arc::new(RwLock::new(v.into())))
    }
}

impl Display for Val {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use Val::*;

        match self {
            Nil => write!(f, "nil"),
            True => write!(f, "T"),
            False => write!(f, "F"),
            Int(n) => write!(f, "{}", &n),
            Float(x) => write!(f, "{}", &x),
            String(ref s) => write!(f, "\"{}\"", s),
            Symbol(ref s) => write!(f, "{}", s),
            List(a) => write_list(&a, f),
            Vector(a) => write_vector(&a, f),
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

impl From<Arc<List>> for Val {
    fn from(a: Arc<List>) -> Val {
        Val::List(a.clone())
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
