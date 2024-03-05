/*!
Types.
*/
use std::fmt::{Display, Formatter, Write};

#[derive(Clone, Debug)]
pub enum Val {
    Symbol(String),
    Float(f64),
    Int(i64),
    List(Vec<Val>),
    Array(Vec<Val>),
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

impl Display for Val {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Val::Symbol(ref s) => write!(f, "{}", s),
            Val::Float(ref x) => write!(f, "{}", x),
            Val::Int(ref i) => write!(f, "{}", i),
            Val::List(v) => write_collection(f, v.as_slice(), "(", ")"),
            Val::Array(v) => write_collection(f, v.as_slice(), "[", "]"),
        }
    }
}
