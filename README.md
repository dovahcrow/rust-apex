rust-apex
========

Rust support for apex, letting you run rust on AWS Lambda without pain!

Please refer to [apex-example](https://github.com/apex/apex/tree/master/_examples/rust) for detail usage.

## minimal scaffold

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
