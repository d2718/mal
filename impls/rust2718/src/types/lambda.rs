/*!
The function type.
*/
use std::{
    cmp::Ordering,
    fmt::{Debug, Display, Formatter},
    sync::Arc,
};

use crate::{types::List, Res};

pub type StaticFunc = &'static dyn Fn(Arc<List>) -> Res;

pub trait Lambda: Display + Debug {
    fn call(&self, args: Arc<List>) -> Res;
}

pub struct Builtin {
    name: &'static str,
    func: StaticFunc,
}

impl Builtin {
    pub fn new(name: &'static str, func: StaticFunc) -> Builtin {
        Builtin { name, func }
    }
}

impl PartialEq for Builtin {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
impl Eq for Builtin {}

impl Ord for Builtin {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name.cmp(other.name)
    }
}
impl PartialOrd for Builtin {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Lambda for Builtin {
    fn call(&self, args: Arc<List>) -> Res {
        (self.func)(args)
    }
}

impl Display for Builtin {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{} (builtin function)>", self.name)
    }
}

impl Debug for Builtin {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}
