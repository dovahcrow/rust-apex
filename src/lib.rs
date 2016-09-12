extern crate serde;
extern crate serde_json;

use std::io::stdin;
use std::error::Error;

use serde::{Serialize, Deserialize};
use serde_json::{from_reader, to_string};

use types::{Context, Input, Output};

mod types;

pub trait Handler<I: Deserialize, O: Serialize> {
    fn handle(&self, I, Context) -> Result<O, Box<Error>>;
}

impl<I: Deserialize, O: Serialize, F> Handler<I, O> for F
    where F: Fn(I, Context) -> Result<O, Box<Error>>
{
    fn handle(&self, ipt: I, ctx: Context) -> Result<O, Box<Error>> {
        self(ipt, ctx)
    }
}

pub fn handle_func<I: Deserialize, O: Serialize, H: Handler<I, O>>(h: H) {
    loop {
        let i: Result<Input<I>, _> = from_reader(stdin());
        match i {
            Ok(ipt) => {
                match h.handle(ipt.event, ipt.context) {
                    Ok(r) => {
                        let s = to_string(&Output::Value(r)).unwrap();
                        println!("{}", s);
                    }
                    Err(e) => {
                        let s = to_string(&Output::Error::<O>(format!("{}", e))).unwrap();
                        println!("{}", s);
                    }
                }
            }
            Err(e) => {
                println!("error: {}", e);
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
