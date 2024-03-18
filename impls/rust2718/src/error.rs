/*!
Our error type.
*/
use std::{
    borrow::Cow,
    fmt::{Display, Formatter},
};

use crate::types::Val;

#[derive(Debug)]
pub struct MalErr {
    pub msg: Cow<'static, str>,
    pub context: Vec<Cow<'static, str>>,
}

impl MalErr {
    pub fn wrap<C>(self, msg: C) -> MalErr
    where
        Cow<'static, str>: From<C>,
    {
        let mut e = self;
        e.context.push(msg.into());
        e
    }

    pub fn in_form(res: Result<Val, MalErr>, val: Val) -> Result<Val, MalErr> {
        match res {
            Ok(v) => Ok(v),
            Err(mut e) => {
                e.context.push(format!("in form {}", &val).into());
                Err(e)
            }
        }
    }
}

pub fn err<C>(msg: C) -> MalErr
where
    Cow<'static, str>: From<C>,
{
    MalErr {
        msg: msg.into(),
        context: Vec::new(),
    }
}

pub fn rerr<C, T>(msg: C) -> Result<T, MalErr>
where
    Cow<'static, str>: From<C>,
{
    Err(err(msg))
}

impl Display for MalErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for line in self.context.iter().rev() {
            writeln!(f, "! {}", line)?;
        }
        writeln!(f, "ERROR: {}", &self.msg)
    }
}
