use rust2718::read::run;

fn start_logging() {
    use tracing_subscriber::{fmt, prelude::*, EnvFilter};

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();
}

fn main() {
    start_logging();
    run();
}
