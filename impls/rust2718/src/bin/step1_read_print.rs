use rust2718::{printer::pr_str, reader::Reader, types::Val, MalErr};

fn read(text: &str) -> Result<Val, MalErr> {
    let mut r = Reader::tokenize(text);
    r.read_form()?
        .ok_or(MalErr::ReadErr("nothing to read".into()))
}
fn eval<P>(p: P) -> P {
    p
}
fn print(val: &Val) -> String {
    val.to_string()
}
fn rep(s: String) -> String {
    match read(&s) {
        Ok(val) => print(&eval(val)),
        Err(e) => e.to_string(),
    }
}

fn main() {
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
