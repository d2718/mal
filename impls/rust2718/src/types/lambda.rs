/*!
The function type.
*/
use std::{
    cmp::Ordering,
    fmt::{Debug, Display, Formatter},
    sync::{Arc, RwLock},
};

use crate::{env::Env, eval::eval, types::List, Res, Val};

pub type StaticFunc = dyn Fn(Arc<List>) -> Res;

pub trait Lambda: Display + Debug {
    fn call(&self, envt: &Arc<Env>, args: Arc<List>) -> Res;
}

pub struct Builtin {
    name: &'static str,
    func: Arc<StaticFunc>,
}

impl Builtin {
    pub fn new(name: &'static str, func: &'static StaticFunc) -> Builtin {
        Builtin {
            name,
            func: Arc::new(func),
        }
    }
}

impl PartialEq for Builtin {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
impl Eq for Builtin {}

impl Ord for Builtin {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name.cmp(other.name)
    }
}
impl PartialOrd for Builtin {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Lambda for Builtin {
    fn call(&self, _: &Arc<Env>, args: Arc<List>) -> Res {
        (self.func)(args)
    }
}

impl Display for Builtin {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{} (builtin function)>", self.name)
    }
}

impl Debug for Builtin {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

pub struct Function {
    name: RwLock<Option<Arc<str>>>,
    args: Vec<Arc<str>>,
    envt: Arc<Env>,
    form: Val,
}

impl Function {
    pub fn define(args: Vec<Arc<str>>, envt: &Arc<Env>, form: Val) -> Function {
        Function {
            name: RwLock::new(None),
            args,
            envt: envt.clone(),
            form,
        }
    }

    pub fn set_name(&self, name: &Arc<str>) {
        *self.name.write().unwrap() = Some(name.clone());
    }
}

impl Lambda for Function {
    fn call(&self, envt: &Arc<Env>, args: Arc<List>) -> Res {
        let mut bindings: Vec<(Arc<str>, Val)> = Vec::with_capacity(self.args.len());
        let mut args = args.clone();
        for sym in self.args.iter() {
            let val = eval(envt, args.pop()?)?;
            bindings.push((sym.clone(), val));
        }

        let fn_env = Env::binding(&self.envt, bindings);
        eval(&fn_env, self.form.clone())
    }
}

impl Display for Function {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self.name.read().unwrap() {
            None => write!(f, "<(anonymous interpreted function)>"),
            Some(ref a) => write!(f, "<{} (interpreted function)>", a),
        }
    }
}

impl Debug for Function {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}
