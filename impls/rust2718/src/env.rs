/*!
The * ** *** ENVIRONMENT *** ** *
*/
use std::{
    collections::BTreeMap,
    sync::{Arc, RwLock},
};

use crate::{
    hard::math,
    types::{Builtin, Fun, Val, Value},
    MalErr, Res,
};

const DEFAULT_ENV: &[(&str, &Builtin)] = &[
    ("+", &math::add),
    ("-", &math::sub),
    ("*", &math::mul),
    ("/", &math::div),
    ("div", &math::idiv),
    ("rem", &math::rem),
];

pub type EnvTree = BTreeMap<Box<str>, Value>;
pub type Envr = Arc<Env>;

pub fn default_env() -> Arc<Env> {
    let data: EnvTree = DEFAULT_ENV
        .iter()
        .copied()
        .map(|(sym, f)| {
            let b: Box<str> = sym.into();
            (b, Val::Fun(Fun::new(sym.to_string(), f)).into())
        })
        .collect();

    let mut env = Env::bare();
    env.data = RwLock::new(data);
    env.into()
}

#[derive(Debug)]
pub struct Env {
    outer: Option<Arc<Env>>,
    data: RwLock<EnvTree>,
}

impl Env {
    fn bare() -> Env {
        Env {
            outer: None,
            data: RwLock::new(EnvTree::new()),
        }
    }

    pub fn empty() -> Arc<Env> {
        Env::bare().into()
    }

    pub fn child(self: &Arc<Self>) -> Arc<Env> {
        let mut env = Env::bare();
        env.outer = Some(self.clone());
        env.into()
    }

    pub fn set<S: AsRef<str>>(&self, key: S, val: &Value) {
        self.data
            .write()
            .unwrap()
            .insert(key.as_ref().into(), val.clone());
    }

    pub fn find<S: AsRef<str>>(self: Arc<Self>, key: S) -> Option<Arc<Env>> {
        match self.data.read().unwrap().get(key.as_ref()) {
            Some(_) => Some(self.clone()),
            None => match &self.outer {
                Some(env) => env.clone().find(key.as_ref()),
                None => None,
            },
        }
    }

    // pub fn get<S: AsRef<str>>(self: Arc<Self>, key: S) -> Res {
    //     match self.find(key.as_ref()) {
    //         None => Err(MalErr::ExecErr("no such symbol")),
    //         Some(env) =>
    //     }
    // }

    pub fn get<S: AsRef<str>>(self: &Arc<Self>, key: S) -> Res {
        if let Some(val) = self.data.read().unwrap().get(key.as_ref()) {
            return Ok(val.clone());
        } else {
            match &self.outer {
                Some(env) => env.get(key.as_ref()),
                None => Err(MalErr::ExecErr("no such symbol".into())),
            }
        }
    }
}
