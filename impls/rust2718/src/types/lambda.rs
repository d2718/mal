/*!
The function type is going to require quite a bit of infrastructure.
*/
use std::{
    cmp::Ordering,
    fmt::{Display, Debug, Formatter},
    hash::{Hash, Hasher},
    io::Read,
};

use super::Val;
use crate::MalErr;

pub trait Lambda {}

impl<F> Lambda for F where F: Fn(&[Val]) -> Result<Val, MalErr> + Sync {}

struct FunHash([u8; 32]);

impl Display for FunHash {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut a = [0u8; 8];
        let mut b = [0u8; 8];
        let mut c = [0u8; 8];
        let mut d = [0u8; 8];
    }
}

fn new_hash() -> [u8; 32] {
    let mut a: [u8; 32] = [0u8; 32];
    let f = std::fs::File::open("/dev/urandom")
        .wrap_err("unable to open /dev/urandom")
        .unwrap();
    f.read_exact(&mut a)
        .wrap_err("error reading from /dev/urandom")
        .unwrap();
    a
}

#[derive(Clone, PartialEq, Eq)]
pub struct Fun {
    name: String,
    hash: [u8; 32],
    lambda: Box<dyn Lambda>,
}

impl Ord for Fun {
    fn cmp(&self, other: &Self) -> Ordering {
        self.hash.cmp(&other.hash)
    }
}

impl PartialOrd for Fun {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Hash for Fun {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.hash.hash(state)
    }
}

impl Display for Fun {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut a: 
    }
}

impl Fun {
    pub fn new(name: String, f: &dyn Lambda) -> Fun {
        let lambda = Box::new(f.clone());
        let hash = new_hash();
        Fun { name, hash, lambda }
    }
}
