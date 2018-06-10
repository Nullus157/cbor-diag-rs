#[macro_use]
extern crate indoc;
#[macro_use]
extern crate pretty_assertions;

extern crate cbor_diag;

use cbor_diag::{IntegerWidth, Value};

#[macro_use]
mod utils;

// CBOR diagnostic notation provides for no way to encode the width of the
// length value of a byte string, so unfortunately roundtripping cannot be
// supported.
//
// Maybe I can just extend diagnostic notation for this?

testcases! {
    mod diag {
        empty(diag2value, value2diag) {
            Value::ByteString {
                data: vec![],
                bitwidth: Some(IntegerWidth::Unknown),
            },
            "h''",
        }

        hello(diag2value, value2diag) {
            Value::ByteString {
                data: b"hello"[..].into(),
                bitwidth: Some(IntegerWidth::Unknown),
            },
            "h'68656c6c6f'",
        }
    }

    mod tiny {
        empty(hex2value, value2hex) {
            Value::ByteString {
                data: vec![],
                bitwidth: Some(IntegerWidth::Zero),
            },
            indoc!(r#"
                40  # bytes(0)
                    # ""
            "#)
        }

        hello(hex2value, value2hex) {
            Value::ByteString {
                data: b"hello"[..].into(),
                bitwidth: Some(IntegerWidth::Zero),
            },
            indoc!(r#"
                45            # bytes(5)
                   68656c6c6f # "hello"
            "#)
        }
    }
}
