/*!
The classic singly-linked list.
*/
use std::{ops::Deref, sync::Arc};

use crate::{error::rerr, types::Val, MalErr, Res};

#[derive(Debug, PartialEq)]
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

    pub fn is_last(self: &Arc<List>) -> bool {
        if let List::Node { next, .. } = self.deref() {
            next.is_empty()
        } else {
            false
        }
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

    pub fn car(self: &Arc<List>) -> Res {
        match self.deref() {
            List::Nil => rerr("car expects a non-empty list"),
            List::Node { val, .. } => Ok(val.clone()),
        }
    }

    pub fn cdr(self: &Arc<List>) -> Result<Arc<List>, MalErr> {
        match self.deref() {
            List::Nil => rerr("cdr expects a non-empty list"),
            List::Node { next, .. } => Ok(next.clone()),
        }
    }

    pub fn next(self: &mut Arc<List>) -> Option<Val> {
        match self.deref().deref() {
            List::Nil => None,
            List::Node { next, val } => {
                let rval = val.clone();
                *self = next.clone();
                Some(rval)
            }
        }
    }

    pub fn pop(self: &mut Arc<List>) -> Res {
        match self.deref().deref() {
            List::Nil => rerr("list is empty"),
            List::Node { next, val } => {
                let rval = val.clone();
                *self = next.clone();
                Ok(rval)
            }
        }
    }

    pub fn len(self: &Arc<List>) -> i64 {
        let mut list = self.clone();
        let mut n = 0i64;
        while list.next().is_some() {
            n += 1;
        }
        n
    }

    pub fn get_n_args(self: &mut Arc<List>, n: usize) -> Result<Vec<Val>, MalErr> {
        let mut v: Vec<Val> = Vec::with_capacity(n);
        for _ in 0..n {
            match self.next() {
                Some(val) => v.push(val),
                None => return rerr("not enough arguments"),
            }
        }

        Ok(v)
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
