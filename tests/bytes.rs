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

        alpha(diag2value, value2diag) {
            Value::ByteString {
                data: b"abcdefghijklmnopqrstuvwxyz"[..].into(),
                bitwidth: Some(IntegerWidth::Unknown),
            },
            "h'6162636465666768696a6b6c6d6e6f707172737475767778797a'",
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

    mod u8 {
        empty(hex2value, value2hex) {
            Value::ByteString {
                data: vec![],
                bitwidth: Some(IntegerWidth::Eight),
            },
            indoc!(r#"
                58 00 # bytes(0)
                      # ""
            "#)
        }

        hello(hex2value, value2hex) {
            Value::ByteString {
                data: b"hello"[..].into(),
                bitwidth: Some(IntegerWidth::Eight),
            },
            indoc!(r#"
                58 05         # bytes(5)
                   68656c6c6f # "hello"
            "#)
        }

        alpha(hex2value, value2hex) {
            Value::ByteString {
                data: b"abcdefghijklmnopqrstuvwxyz"[..].into(),
                bitwidth: Some(IntegerWidth::Eight),
            },
            indoc!(r#"
                58 1a                               # bytes(26)
                   6162636465666768696a6b6c6d6e6f70 # "abcdefghijklmnop"
                   7172737475767778797a             # "qrstuvwxyz"
            "#)
        }

        non_alpha(hex2value, value2hex) {
            Value::ByteString {
                data: vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
                bitwidth: Some(IntegerWidth::Eight),
            },
            indoc!(r#"
                58 0b                     # bytes(11)
                   000102030405060708090a # "\x00\x01\x02\x03\x04\x05\x06\x07\x08\t\n"
            "#)
        }
    }

    mod u16 {
        empty(hex2value, value2hex) {
            Value::ByteString {
                data: vec![],
                bitwidth: Some(IntegerWidth::Sixteen),
            },
            indoc!(r#"
                59 0000 # bytes(0)
                        # ""
            "#)
        }

        hello(hex2value, value2hex) {
            Value::ByteString {
                data: b"hello"[..].into(),
                bitwidth: Some(IntegerWidth::Sixteen),
            },
            indoc!(r#"
                59 0005       # bytes(5)
                   68656c6c6f # "hello"
            "#)
        }

        alpha(hex2value, value2hex) {
            Value::ByteString {
                data: b"abcdefghijklmnopqrstuvwxyz"[..].into(),
                bitwidth: Some(IntegerWidth::Sixteen),
            },
            indoc!(r#"
                59 001a                             # bytes(26)
                   6162636465666768696a6b6c6d6e6f70 # "abcdefghijklmnop"
                   7172737475767778797a             # "qrstuvwxyz"
            "#)
        }
    }

    mod u32 {
        empty(hex2value, value2hex) {
            Value::ByteString {
                data: vec![],
                bitwidth: Some(IntegerWidth::ThirtyTwo),
            },
            indoc!(r#"
                5a 00000000 # bytes(0)
                            # ""
            "#)
        }

        hello(hex2value, value2hex) {
            Value::ByteString {
                data: b"hello"[..].into(),
                bitwidth: Some(IntegerWidth::ThirtyTwo),
            },
            indoc!(r#"
                5a 00000005   # bytes(5)
                   68656c6c6f # "hello"
            "#)
        }

        alpha(hex2value, value2hex) {
            Value::ByteString {
                data: b"abcdefghijklmnopqrstuvwxyz"[..].into(),
                bitwidth: Some(IntegerWidth::ThirtyTwo),
            },
            indoc!(r#"
                5a 0000001a                         # bytes(26)
                   6162636465666768696a6b6c6d6e6f70 # "abcdefghijklmnop"
                   7172737475767778797a             # "qrstuvwxyz"
            "#)
        }
    }

    mod u64 {
        empty(hex2value, value2hex) {
            Value::ByteString {
                data: vec![],
                bitwidth: Some(IntegerWidth::SixtyFour),
            },
            indoc!(r#"
                5b 0000000000000000 # bytes(0)
                                    # ""
            "#)
        }

        hello(hex2value, value2hex) {
            Value::ByteString {
                data: b"hello"[..].into(),
                bitwidth: Some(IntegerWidth::SixtyFour),
            },
            indoc!(r#"
                5b 0000000000000005 # bytes(5)
                   68656c6c6f       # "hello"
            "#)
        }

        alpha(hex2value, value2hex) {
            Value::ByteString {
                data: b"abcdefghijklmnopqrstuvwxyz"[..].into(),
                bitwidth: Some(IntegerWidth::SixtyFour),
            },
            indoc!(r#"
                5b 000000000000001a                 # bytes(26)
                   6162636465666768696a6b6c6d6e6f70 # "abcdefghijklmnop"
                   7172737475767778797a             # "qrstuvwxyz"
            "#)
        }
    }
}
