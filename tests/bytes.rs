#[macro_use]
extern crate indoc;
extern crate cbor_diag;

use cbor_diag::{IntegerWidth, Value};

#[macro_use]
mod utils;

testcases! {
    mod tiny {
        empty {
            Value::ByteString {
                data: vec![],
                bitwidth: Some(IntegerWidth::Zero),
            },
            "h''",
            indoc!(r#"
                40 # bytes(0)
                   # ""
            "#)
        }
    }
}
