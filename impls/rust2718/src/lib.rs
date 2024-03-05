pub mod printer;
pub mod reader;
pub mod types;

use std::{
    borrow::Cow,
    fmt::{Display, Formatter},
};

#[derive(Debug)]
pub enum MalErr {
    ReadErr(Cow<'static, str>),
}

impl Display for MalErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for MalErr {}
