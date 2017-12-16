extern crate failure;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate rust_apex;

use std::collections::HashMap;
use serde_json::{Value, to_string};
use rust_apex::Context;
use failure::Error;

lambda_entry!(handle);

fn handle(input: Payload, _: Context) -> Result<Value, Error> {
    Ok(json!({
        "statusCode": "200",
        "headers": {},
        "body": to_string(&json!({
            "status": "success", 
            "input_body": input.body,
            "input_headers": input.headers,
        })).unwrap()
    }))
}

#[derive(Deserialize, Serialize)]
struct Payload {
    body: String,
    headers: HashMap<String, String>,
}