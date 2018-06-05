extern crate cbor_diag;

use cbor_diag::{Simple, Value};

#[macro_use]
mod utils;

testcases! {
    false_ {
        Value::Simple(Simple::FALSE),
        "false",
        "f4 # false, simple(20)",
    }

    true_ {
        Value::Simple(Simple::TRUE),
        "true",
        "f5 # true, simple(21)",
    }

    null {
        Value::Simple(Simple::NULL),
        "null",
        "f6 # null, simple(22)",
    }

    undefined {
        Value::Simple(Simple::UNDEFINED),
        "undefined",
        "f7 # undefined, simple(23)",
    }

    simple_16 {
        Value::Simple(Simple(16)),
        "simple(16)",
        "f0 # unassigned, simple(16)",
    }

    simple_24 {
        Value::Simple(Simple(24)),
        "simple(24)",
        "f8 18 # reserved, simple(24)",
    }

    simple_255 {
        Value::Simple(Simple(255)),
        "simple(255)",
        "f8 ff # unassigned, simple(255)",
    }
}
