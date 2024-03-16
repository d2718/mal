use std::sync::Arc;

use rust2718::{env::Env, error::err, eval::eval, read::Reader, ErrType, MalErr, Val};

fn read(text: &str) -> Result<Val, MalErr> {
    let mut r = Reader::default();
    r.tokenize(text);
    r.read_form()?.ok_or(err(ErrType::Read, "nothing to read"))
}

fn print(val: &Val) -> String {
    val.to_string()
}

fn rep(s: String, envt: &Arc<Env>) -> String {
    match read(&s).and_then(|v| eval(envt, v)) {
        Ok(val) => print(&val),
        Err(e) => format!("{:?}", &e),
    }
}

fn start_logging() {
    use tracing_subscriber::{fmt, prelude::*, EnvFilter};

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();
}

fn main() {
    use rustyline::{config::EditMode, DefaultEditor};
    start_logging();

    let rl_conf = rustyline::Config::builder()
        .auto_add_history(true)
        .edit_mode(EditMode::Emacs)
        .build();
    let mut rl = DefaultEditor::with_config(rl_conf).unwrap();
    let env = Env::default();

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
