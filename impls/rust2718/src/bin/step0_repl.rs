fn read<P>(p: P) -> P {
    p
}
fn eval<P>(p: P) -> P {
    p
}
fn print<P>(p: P) -> P {
    p
}
fn rep(s: String) -> String {
    let r = print(eval(read(s)));
    r
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
