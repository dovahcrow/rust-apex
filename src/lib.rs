extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate failure;

use std::io::stdin;

use failure::Error;
use serde::{Serialize, Deserialize};
use serde_json::{from_str, to_string};

use types::{Input, Output};

pub use types::Context;

mod types;
mod macros;

pub trait Handler<I: for<'de> Deserialize<'de>, O: Serialize, E: Into<Error>> {
    fn handle(&self, I, Context) -> Result<O, E>;
}

impl<I, O, E, F> Handler<I, O, E> for F
where
    I: for<'de> Deserialize<'de>,
    O: Serialize,
    E: Into<Error>,
    F: Fn(I, Context) -> Result<O, E>,
{
    fn handle(&self, ipt: I, ctx: Context) -> Result<O, E> {
        self(ipt, ctx)
    }
}

pub fn run<I, O, E, H>(h: H)
where
    I: for<'de> Deserialize<'de>,
    O: Serialize,
    E: Into<Error>,
    H: Handler<I, O, E>,
{
    let mut buf = String::new();
    loop {

        // use `from_reader(stdin())` here is not correct.
        if let Err(e) = stdin().read_line(&mut buf) {
            return_error::<O, _, _>(None, e);
            break;
        }
        let i: Result<Input<I>, _> = from_str(&buf);

        match i {
            Ok(ipt) => {
                match h.handle(ipt.event, ipt.context) {
                    Ok(r) => return_success(ipt.id, r),
                    Err(e) => return_error::<O, _, _>(ipt.id, e),
                }
            }
            Err(e) => return_error::<O, _, _>(None, e),
        }

        buf.clear();
    }
}

fn return_error<O: Serialize, E: Into<Error>, K: Into<Option<String>>>(id: K, e: E) {
    let id = id.into();

    let s = to_string(&Output::Error::<O> {
        id: id,
        error: format!("{}", e.into()),
    }).unwrap();
    println!("{}", s);
}

fn return_success<O: Serialize>(id: String, r: O) {
    let s = to_string(&Output::Value { id: id, value: r }).unwrap();
    println!("{}", s);
}
