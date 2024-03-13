use rust2718::{
    error::err,
    eval::{eval, Env},
    read::Reader,
    ErrType, MalErr, Val,
};

fn read(text: &str) -> Result<Val, MalErr> {
    let mut r = Reader::default();
    r.tokenize(text);
    r.read_form()?.ok_or(err(ErrType::Read, "nothing to read"))
}

fn print(val: &Val) -> String {
    val.to_string()
}

fn rep(s: String, envt: &Env) -> String {
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
    start_logging();
    let mut rl = rustyline::DefaultEditor::new().unwrap();
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
