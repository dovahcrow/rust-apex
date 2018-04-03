rust-apex
========

This library provides Rust support for apex, letting you run rust on AWS Lambda without pain!

A example with apex configuration for quick start can be found at [apex-example](https://github.com/apex/apex/tree/master/_examples/rust) .

Here is a simple example for minimal scaffold:
```rust
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
```

More running examples can be found in `examples` directory.
