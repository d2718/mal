/*!
Then ENVIRONMENT.
*/
use std::{
    collections::BTreeMap,
    ops::Deref,
    sync::{Arc, RwLock},
};

use crate::{error::err, types::Val, Res};

#[derive(Debug)]
pub struct Env {
    outer: Option<Arc<Env>>,
    map: RwLock<BTreeMap<Box<str>, Val>>,
}

impl Env {
    pub fn child_of(outer: &Arc<Env>) -> Arc<Env> {
        Env {
            outer: Some(outer.clone()),
            map: RwLock::new(BTreeMap::default()),
        }
        .into()
    }

    pub fn binding(outer: &Arc<Env>, bindings: Vec<(Arc<str>, Val)>) -> Arc<Env> {
        let envt = Env::child_of(outer);
        for (k, v) in bindings.into_iter() {
            envt.set(&k, v);
        }
        envt
    }

    fn self_get(self: &Arc<Env>, key: &str) -> Option<Val> {
        self.map.read().unwrap().get(key).cloned()
    }

    pub fn get<S: AsRef<str>>(self: &Arc<Env>, s: S) -> Res {
        let s = s.as_ref();
        let envt = self
            .find(s)
            .ok_or_else(|| err(format!("'{}' not found", s)))?;
        envt.self_get(s)
            .ok_or_else(|| err(format!("'{}' not found", s)))
    }

    pub fn find(self: &Arc<Env>, key: &str) -> Option<Arc<Env>> {
        if self.self_get(key).is_some() {
            Some(self.clone())
        } else {
            match &self.deref().outer {
                Some(a) => a.find(key),
                None => None,
            }
        }
    }

    pub fn set(self: &Arc<Env>, key: &str, v: Val) {
        self.deref().map.write().unwrap().insert(key.into(), v);
    }

    pub fn default() -> Arc<Env> {
        use crate::types::{builtin, builtin::math, Builtin};

        let mut map = BTreeMap::default();
        for (name, func) in builtin::BUILTINS.iter().chain(math::BUILTINS.iter()) {
            let f = Builtin::new(name, func);
            let name: Box<str> = (*name).into();
            map.insert(name, f.into());
        }

        Env {
            outer: None,
            map: RwLock::new(map),
        }
        .into()
    }
}
