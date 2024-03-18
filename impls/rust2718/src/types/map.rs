/*!
The MAP type.
*/

use std::{
    collections::BTreeMap,
    sync::{Arc, RwLock},
};

use ordered_float::OrderedFloat;

use crate::{error::rerr, MalErr, Res, Val};

#[allow(dead_code)]
#[derive(Clone, Debug, PartialOrd, PartialEq, Ord, Eq)]
enum Key {
    Int(i64),
    Float(OrderedFloat<f64>),
    Keyword(Arc<str>),
    String(Arc<str>),
    Symbol(Arc<str>),
}

impl From<&Key> for Val {
    fn from(k: &Key) -> Val {
        match k {
            Key::Int(i) => Val::Int(*i),
            Key::Float(x) => Val::Float(*x),
            Key::Keyword(a) => Val::String(a.clone()),
            Key::String(a) => Val::String(a.clone()),
            Key::Symbol(a) => Val::Symbol(a.clone()),
        }
    }
}

impl TryFrom<Val> for Key {
    type Error = MalErr;

    fn try_from(v: Val) -> Result<Self, Self::Error> {
        let key = match v {
            Val::Int(n) => Key::Int(n),
            Val::Float(x) => Key::Float(x),
            Val::String(a) => Key::String(a.clone()),
            Val::Symbol(a) => Key::Symbol(a.clone()),
            _ => return rerr("invalid Map key"),
        };
        Ok(key)
    }
}

#[derive(Debug)]
pub struct Map {
    map: RwLock<BTreeMap<Key, Val>>,
}

impl Default for Map {
    fn default() -> Self {
        let map = RwLock::new(BTreeMap::default());
        Self { map }
    }
}

impl Map {
    pub fn insert(self: &Arc<Map>, k: Val, v: Val) -> Res {
        let key = Key::try_from(k)?;

        self.map.write().unwrap().insert(key, v.clone());
        Ok(self.clone().into())
    }

    pub fn get(self: &Arc<Map>, k: Val) -> Option<Val> {
        let k = Key::try_from(k).ok()?;

        self.map.read().unwrap().get(&k).cloned()
    }

    pub fn iter(self: &Arc<Map>) -> MapIter {
        let guard = self.map.read().unwrap();
        let mut values: Vec<(Val, Val)> = Vec::with_capacity(guard.len());
        for (k, v) in guard.iter() {
            let key: Val = k.into();
            let val: Val = v.clone();
            values.push((key, val))
        }

        MapIter { values }
    }
}

impl PartialEq for Map {
    fn eq(&self, other: &Self) -> bool {
        let s = self.map.read().unwrap();
        let t = other.map.read().unwrap();
        if s.len() != t.len() {
            return false;
        }

        for (k, v) in s.iter() {
            if t.get(k) != Some(v) {
                return false;
            }
        }

        true
    }
}

pub struct MapIter {
    values: Vec<(Val, Val)>,
}

impl Iterator for MapIter {
    type Item = (Val, Val);

    fn next(&mut self) -> Option<Self::Item> {
        self.values.pop()
    }
}
