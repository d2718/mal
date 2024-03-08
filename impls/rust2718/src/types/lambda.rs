/*!
The function type is going to require quite a bit of infrastructure.
*/
use std::{
    cmp::Ordering,
    fmt::{Debug, Display, Formatter, Write},
    hash::{Hash, Hasher},
    io::Read,
    sync::Arc,
};

use super::{Val, Value};
use crate::{MalErr, Res};

pub type Lambda = dyn Fn(&[Value]) -> Res;

#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
struct FunHash([u8; 32]);

impl Display for FunHash {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut a = [0u8; 8];
        let mut b = [0u8; 8];
        let mut c = [0u8; 8];
        let mut d = [0u8; 8];
        a.as_mut_slice().copy_from_slice(&self.0[0..8]);
        b.as_mut_slice().copy_from_slice(&self.0[8..16]);
        c.as_mut_slice().copy_from_slice(&self.0[16..24]);
        d.as_mut_slice().copy_from_slice(&self.0[24..32]);
        write!(
            f,
            "{:016x}{:016x}{:016x}{:016x}",
            &u64::from_ne_bytes(a),
            &u64::from_ne_bytes(b),
            &u64::from_ne_bytes(c),
            &u64::from_ne_bytes(d),
        )
    }
}

fn new_hash() -> FunHash {
    let mut a: [u8; 32] = [0u8; 32];
    let mut f = std::fs::File::open("/dev/urandom")
        .map_err(|e| format!("unable to open /dev/urandom: {}", &e))
        .unwrap();
    f.read_exact(&mut a)
        .map_err(|e| format!("error reading from /dev/urandom: {}", &e))
        .unwrap();
    FunHash(a)
}

pub struct Fun {
    name: String,
    hash: FunHash,
    lambda: Box<Lambda>,
}

impl PartialEq for Fun {
    fn eq(&self, other: &Self) -> bool {
        self.hash == other.hash
    }
}

impl Eq for Fun {}

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
        self.hash.0.hash(state)
    }
}

impl Display for Fun {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<Fun {}>", &self.name)
    }
}

impl Debug for Fun {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<Fun {} 0x{}", &self.name, &self.hash)
    }
}

impl Fun {
    pub fn new(name: String, f: &Lambda) -> Fun {
        let lambda = Box::new(f);
        let hash = new_hash();
        Fun { name, hash, lambda }
    }

    pub fn call(&self, args: &[Value]) -> Res {
        (*self.lambda)(args)
    }

    pub fn fun(&self) -> &Lambda {
        &*self.lambda
    }
}
