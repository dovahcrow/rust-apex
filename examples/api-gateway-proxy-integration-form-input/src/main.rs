extern crate failure;
#[macro_use]
extern crate rust_apex;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate serde_bytes;
extern crate url;

use std::collections::HashMap;
use serde_json::{Value, to_string};
use rust_apex::Context;
use failure::Error;
use url::form_urlencoded::parse;

lambda_entry!(handle);

fn handle(input: Payload, _: Context) -> Result<Value, Error> {
    let content_type = input.headers.get("Content-Type").unwrap();

    if content_type.starts_with("application/x-www-form-urlencoded") {
        let fields: Vec<_> = parse(&input.body).collect();
        Ok(json!({
            "statusCode": "200",
            "headers": {},
            "body": to_string(&json!({
                "status": "success", 
                "input_fields": fields,
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