use tracing::{Level, Metadata, Subscriber};
use tracing_subscriber::{
    layer::{Context, Layer},
    prelude::*,
};

pub mod env;
pub mod error;
pub mod eval;
pub mod read;
pub mod types;

pub use crate::error::{ErrType, MalErr};
pub use crate::types::Val;

pub type Res = Result<Val, MalErr>;

struct LogFilter {
    lvl: Level,
}

impl LogFilter {
    pub fn new(lvl: Level) -> LogFilter {
        LogFilter { lvl }
    }
}

impl<S: Subscriber> Layer<S> for LogFilter {
    fn enabled(&self, meta: &Metadata<'_>, _: Context<'_, S>) -> bool {
        if meta.level() > &self.lvl {
            return false;
        }

        if meta.target().starts_with("mal") || meta.target().starts_with("step") {
            return true;
        }

        false
    }
}

pub fn start_logging(level: Level) {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(LogFilter::new(level))
        .init();
}
