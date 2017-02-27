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

pub trait Handler<I: Deserialize, O: Serialize, E: Error> {
    fn handle(&self, I, Context) -> Result<O, E>;
}

impl<I: Deserialize, O: Serialize, E: Error, F> Handler<I, O, E> for F
    where F: Fn(I, Context) -> Result<O, E>
{
    fn handle(&self, ipt: I, ctx: Context) -> Result<O, E> {
        self(ipt, ctx)
    }
}

pub fn run<I: Deserialize, O: Serialize, E: Error, H: Handler<I, O, E>>(h: H) {
    let mut buf = String::new();
    loop {

        // use `from_reader(stdin())` here is not correct.
        if let Err(e) = stdin().read_line(&mut buf) {
            return_error::<O, _>(e);
            break;
        }
        let i: Result<Input<I>, _> = from_str(&buf);

        match i {
            Ok(ipt) => {
                match h.handle(ipt.event, ipt.context) {
                    Ok(r) => return_success(r),
                    Err(e) => return_error::<O, _>(e),
                }
            }
            Err(e) => return_error::<O, _>(e),
        }

        buf.clear();
    }
}

fn return_error<O: Serialize, E: Error>(e: E) {
    let s = to_string(&Output::Error::<O>(format!("{}", e))).unwrap();
    println!("{}", s);
}

fn return_success<O: Serialize>(r: O) {
    let s = to_string(&Output::Value(r)).unwrap();
    println!("{}", s);
}
