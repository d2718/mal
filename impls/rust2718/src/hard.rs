/*!
"Hard-coded" or "intrinsic" or "native" functions.
*/
use crate::{types::Val, MalErr, Res};

pub mod math;
/*
fn map_vec(f: &dyn Fn(&[Val]) -> Res, vals: &[Val]) -> Result<Vec<Val>, MalErr> {
    vals.iter().map(|v| f(std::slice::from_ref(v))).collect()
}

pub fn map(args: &[Val]) -> Res {
    let f = match args.get(0) {
        Some(Val::Fun(f)) => f.clone(),
        _ => {
            return Err(MalErr::ArgErr(
                "map requires function as first argument".into(),
            ))
        }
    };

    match args.get(1) {
        Some(Val::List(v)) => Ok(Val::List(map_vec(f.fun(), &v)?)),
        Some(Val::Array(v)) => Ok(Val::Array(map_vec(f.fun(), &v)?)),
        _ => Err(MalErr::ArgErr(
            "map requires list or array as second argument".into(),
        )),
    }
}
*/
