/*!
Our error type.
*/
use std::borrow::Cow;

#[derive(Debug)]
pub enum ErrType {
    Read,
    Arg,
    Type,
    Eval,
}

#[derive(Debug)]
pub struct MalErr {
    pub flavor: ErrType,
    pub msg: Cow<'static, str>,
}

impl MalErr {
    pub fn rread<C, T>(msg: C) -> Result<T, MalErr>
    where
        Cow<'static, str>: From<C>,
    {
        Err(MalErr {
            flavor: ErrType::Read,
            msg: msg.into(),
        })
    }

    pub fn arg<C>(msg: C) -> MalErr
    where
        Cow<'static, str>: From<C>,
    {
        MalErr {
            flavor: ErrType::Arg,
            msg: msg.into(),
        }
    }

    pub fn rarg<C, T>(msg: C) -> Result<T, MalErr>
    where
        Cow<'static, str>: From<C>,
    {
        Err(MalErr::arg(msg))
    }
}

pub fn err<C>(t: ErrType, msg: C) -> MalErr
where
    Cow<'static, str>: From<C>,
{
    MalErr {
        flavor: t,
        msg: msg.into(),
    }
}
