extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use std::io::stdin;
use std::error::Error;

use serde::{Serialize, Deserialize};
use serde_json::{from_str, to_string};

use types::{Input, Output};

pub use types::Context;

mod types;
mod macros;

pub trait Handler<I: for<'de> Deserialize<'de>, O: Serialize, E: Error> {
    fn handle(&self, I, Context) -> Result<O, E>;
}

impl<I: for<'de> Deserialize<'de>, O: Serialize, E: Error, F> Handler<I, O, E> for F
where
    F: Fn(I, Context)
       -> Result<
        O,
        E,
    >,
{
    fn handle(&self, ipt: I, ctx: Context) -> Result<O, E> {
        self(ipt, ctx)
    }
}

pub fn run<I: for<'de> Deserialize<'de>, O: Serialize, E: Error, H: Handler<I, O, E>>(h: H) {
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

fn return_error<O: Serialize, E: Error, K: Into<Option<String>>>(id: K, e: E) {
    let id = id.into();

    let s = to_string(&Output::Error::<O> {
        id: id,
        error: format!("{}", e),
    }).unwrap();
    println!("{}", s);
}

fn return_success<O: Serialize>(id: String, r: O) {
    let s = to_string(&Output::Value { id: id, value: r }).unwrap();
    println!("{}", s);
}
