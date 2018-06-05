extern crate cbor_diag;

use cbor_diag::{Simple, Value};

#[macro_use]
mod utils;

testcases! {
    null {
        Value::Simple(Simple::NULL),
        "null",
        "f6",
    }
}
