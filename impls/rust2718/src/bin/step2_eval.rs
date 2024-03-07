use rust2718::{
    eval::{default_env, eval},
    printer::pr_str,
    reader::Reader,
    types::Val,
    MalErr,
};

use tracing::{event, Level};

fn env_logging() {
    use tracing_subscriber::{fmt, prelude::*, EnvFilter};

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_env("LOG"))
        .init();
    event!(Level::INFO, "logging started");
}

fn read(text: &str) -> Result<Val, MalErr> {
    let mut r = Reader::tokenize(text);
    r.read_form()?
        .ok_or(MalErr::ReadErr("nothing to read".into()))
}

fn print(val: &Val, readably: bool) -> String {
    if readably {
        format!("{:?}", val)
    } else {
        val.to_string()
    }
}
fn rep(s: String) -> String {
    match read(&s) {
        Ok(val) => match eval(val, &default_env()) {
            Ok(val) => print(&val, true),
            Err(e) => format!("{}", &e),
        },
        Err(e) => e.to_string(),
    }
}

fn main() {
    env_logging();
    let mut rl = rustyline::DefaultEditor::new().unwrap();
    loop {
        match rl.readline("user> ") {
            Ok(line) => {
                let v = rep(line);
                println!("{}", &v);
            }
            Err(_) => std::process::exit(0),
        }
    }
}
