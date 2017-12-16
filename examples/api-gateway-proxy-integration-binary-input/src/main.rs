extern crate failure;
#[macro_use]
extern crate serde_derive;
extern crate serde_bytes;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate rust_apex;
extern crate base64;

use std::collections::HashMap;
use serde_json::{Value, to_string};
use rust_apex::Context;
use failure::Error;
use base64::decode;

lambda_entry!(handle);

fn handle(input: Payload, _: Context) -> Result<Value, Error> {
    let content_type = input.headers.get("Content-Type").unwrap();

    if content_type.starts_with("image/jpeg") {
        Ok(json!({
            "statusCode": "200",
            "headers": {},
            "body": to_string(&json!({
                "status": "success", 
                "input_image": decode(&input.body).unwrap(),
                "input_headers": input.headers,
            })).unwrap()
        }))
    } else {
        Ok(
            json!({"statusCode": 400, "headers": {}, "body": "invalid input"}),
        )
    }

}

#[derive(Deserialize, Serialize)]
struct Payload {
    #[serde(with = "serde_bytes")]
    body: Vec<u8>,
    headers: HashMap<String, String>,
}