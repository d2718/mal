pub mod eval;
pub mod hard;
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
    ArgErr(Cow<'static, str>),
    ExecErr(Cow<'static, str>),
    TypeErr(Cow<'static, str>),
}

impl Display for MalErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for MalErr {}

pub type Res = Result<types::Value, MalErr>;

#[cfg(test)]
pub mod test {
    use std::sync::RwLock;

    static LOGGING_STARTED: RwLock<bool> = RwLock::new(false);

    pub fn start_logging() {
        use tracing_subscriber::{fmt, prelude::*, EnvFilter};

        if *LOGGING_STARTED.read().unwrap() {
            return;
        }

        let mut has_started = LOGGING_STARTED.write().unwrap();
        if *has_started {
            return;
        }

        tracing_subscriber::registry()
            .with(fmt::layer())
            .with(EnvFilter::from_env("LOG"))
            .init();

        *has_started = true;
    }
}
