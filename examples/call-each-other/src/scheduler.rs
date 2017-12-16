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

use serde_json::{Value, to_vec};
use rust_apex::Context;
use failure::Error;

use rusoto_core::{default_tls_client, DefaultCredentialsProvider, Region};
use rusoto_lambda::{Lambda, LambdaClient, InvocationRequest};

lambda_entry!(handle);

fn handle(_: Value, _: Context) -> Result<Value, Error> {

    let message = message::Message::Joy { time: 1440 };

    let req = InvocationRequest {
        function_name: "TheWorkerFuncName".to_string(),
        payload: Some(to_vec(&message).unwrap()),
        ..Default::default()
    };

    // calling lambda
    let credencial = DefaultCredentialsProvider::new().unwrap();
    let client = LambdaClient::new(default_tls_client().unwrap(), credencial, Region::UsWest2);

    // Currently you cannot use "?" if your invocation type is "Event".
    // Rusoto will return Err(Unkown("")) due to a bug for "Event" type invocation, see rusoto issue#893
    let _ = client.invoke(&req)?;

    Ok(json!({"status": "success"}))
}