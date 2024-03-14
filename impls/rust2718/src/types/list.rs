/*!
The classic singly-linked list.
*/
use std::{ops::Deref, sync::Arc};

use crate::{
    types::{Lambda, Val},
    MalErr,
};

#[derive(Debug)]
pub enum List {
    Node { val: Val, next: Arc<List> },
    Nil,
}

impl List {
    pub fn empty() -> Arc<List> {
        Arc::new(List::Nil)
    }

    pub fn is_empty(self: &Arc<List>) -> bool {
        matches!(&(**self), List::Nil)
    }

    pub fn cons<V>(self: &Arc<List>, v: V) -> Arc<List>
    where
        Val: From<V>,
    {
        Arc::new(List::Node {
            val: v.into(),
            next: self.clone(),
        })
    }

    pub fn car(self: &Arc<List>) -> Option<Val> {
        match self.deref() {
            List::Nil => None,
            List::Node { val, .. } => Some(val.clone()),
        }
    }

    pub fn cdr(self: &Arc<List>) -> Arc<List> {
        match self.deref() {
            List::Nil => self.clone(),
            List::Node { next, .. } => next.clone(),
        }
    }

    pub fn next(self: &mut Arc<List>) -> Option<Val> {
        match self.deref().deref() {
            List::Nil => return None,
            List::Node { next, val } => {
                let rval = val.clone();
                *self = next.clone();
                Some(rval)
            }
        }
    }

    pub fn get_n_args(self: &mut Arc<List>, n: usize) -> Result<Vec<Val>, MalErr> {
        let mut v: Vec<Val> = Vec::with_capacity(n);
        for _ in 0..n {
            match self.next() {
                Some(val) => v.push(val),
                None => return MalErr::arg("not enough arguments"),
            }
        }

        Ok(v)
    }

    pub fn map<L: Lambda>(self: &Arc<List>, f: &dyn Lambda) -> Result<Arc<List>, MalErr> {
        let mut a = self.clone();
        let mut temp: Vec<Val> = Vec::new();

        while !a.is_empty() {
            temp.push(f.call(a.clone())?);
            a = a.cdr();
        }

        let mut a = List::empty();
        while let Some(v) = temp.pop() {
            a = a.cons(v);
        }

        Ok(a)
    }

    pub fn from_val(v: Val) -> Arc<List> {
        List::empty().cons(v)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn list_nexting() {
        let mut a = List::empty().cons(34).cons(true).cons(12).cons(-17);
        let b = a.clone();
        println!("{}", Val::from(a.clone()));

        for _ in 0..5 {
            let v = a.next();
            println!("{:?}: {}", &v, Val::from(a.clone()));
        }
        println!("{}", Val::from(b));
    }
}
