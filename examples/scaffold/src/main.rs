extern crate failure;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate rust_apex;

use serde_json::Value;
use rust_apex::Context;
use failure::Error;

lambda_entry!(handle);

fn handle(input: Value, _: Context) -> Result<Value, Error> {
    Ok(json!({
        "input": input
    }))
}