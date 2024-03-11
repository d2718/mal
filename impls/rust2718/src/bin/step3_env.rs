use rust2718::{
    env::Envr,
    eval::{default_env, eval},
    printer::pr_str,
    reader::Reader,
    types::Value,
    MalErr, Res,
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

fn read(text: &str) -> Res {
    let mut r = Reader::tokenize(text);
    r.read_form()?
        .ok_or(MalErr::ReadErr("nothing to read".into()))
}

fn print(val: &Value, readably: bool) -> String {
    if readably {
        format!("{:?}", val)
    } else {
        val.to_string()
    }
}
fn rep(s: String, env: &Envr) -> String {
    match read(&s) {
        Ok(val) => match eval(val, env) {
            Ok(val) => print(&val, true),
            Err(e) => format!("{}", &e),
        },
        Err(e) => e.to_string(),
    }
}

fn main() {
    use rustyline::{config::EditMode, Config, DefaultEditor};
    env_logging();

    let rl_config = rustyline::Config::builder()
        .auto_add_history(true)
        .edit_mode(EditMode::Emacs)
        .build();
    let mut rl = DefaultEditor::with_config(rl_config).unwrap();
    let env = env::default_env();

    loop {
        match rl.readline("user> ") {
            Ok(line) => {
                let v = rep(line, &env);
                println!("{}", &v);
            }
            Err(_) => std::process::exit(0),
        }
    }
}
