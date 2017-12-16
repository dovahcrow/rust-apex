extern crate failure;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate rust_apex;

extern crate rusoto_core;
extern crate rusoto_lambda;

mod message;

use serde_json::Value;
use rust_apex::Context;
use failure::Error;

lambda_entry!(handle);

fn handle(message: message::Message, _: Context) -> Result<Value, Error> {
    let resp = match message {
        message::Message::Job { name } => {
            json!({
                "status": format!("doing job {}!", name)
            })
        }
        message::Message::Joy { time } => {
            json!({
                "status": format!("Chilling for {}s!", time)
            })
        }
    };
    Ok(resp)
}