#[macro_use]
extern crate indoc;
#[macro_use]
extern crate pretty_assertions;

extern crate cbor_diag;

use cbor_diag::{IntegerWidth, Value};

#[macro_use]
mod utils;

// CBOR diagnostic notation provides for no way to encode the width of the
// length value of a string, so unfortunately roundtripping cannot be
// supported.
//
// Maybe I can just extend diagnostic notation for this?

testcases! {
    mod diag {
        empty(diag2value, value2diag) {
            Value::String {
                data: "".into(),
                bitwidth: Some(IntegerWidth::Unknown),
            },
            r#""""#,
        }

        hello(diag2value, value2diag) {
            Value::String {
                data: "hello".into(),
                bitwidth: Some(IntegerWidth::Unknown),
            },
            r#""hello""#,
        }

        alpha(diag2value, value2diag) {
            Value::String {
                data: "abcdefghijklmnopqrstuvwxyz".into(),
                bitwidth: Some(IntegerWidth::Unknown),
            },
            r#""abcdefghijklmnopqrstuvwxyz""#,
        }

        non_alpha(diag2value, value2diag) {
            Value::String {
                data: "\u{1F1F3}\u{1F1FF}".into(),
                bitwidth: Some(IntegerWidth::Unknown),
            },
            "\"\u{1F1F3}\u{1F1FF}\"",
        }

        escaped(diag2value, value2diag) {
            Value::String {
                data: "\\\"".into(),
                bitwidth: Some(IntegerWidth::Unknown),
            },
            r#""\\\"""#,
        }
    }

    mod tiny {
        empty(hex2value, value2hex) {
            Value::String {
                data: "".into(),
                bitwidth: Some(IntegerWidth::Zero),
            },
            indoc!(r#"
                60  # string(0)
                    # ""
            "#)
        }

        hello(hex2value, value2hex) {
            Value::String {
                data: "hello".into(),
                bitwidth: Some(IntegerWidth::Zero),
            },
            indoc!(r#"
                65            # string(5)
                   68656c6c6f # "hello"
            "#)
        }
    }

    mod u8 {
        empty(hex2value, value2hex) {
            Value::String {
                data: "".into(),
                bitwidth: Some(IntegerWidth::Eight),
            },
            indoc!(r#"
                78 00 # string(0)
                      # ""
            "#)
        }

        hello(hex2value, value2hex) {
            Value::String {
                data: "hello".into(),
                bitwidth: Some(IntegerWidth::Eight),
            },
            indoc!(r#"
                78 05         # string(5)
                   68656c6c6f # "hello"
            "#)
        }

        alpha(hex2value, value2hex) {
            Value::String {
                data: "abcdefghijklmnopqrstuvwxyz".into(),
                bitwidth: Some(IntegerWidth::Eight),
            },
            indoc!(r#"
                78 1a                               # string(26)
                   6162636465666768696a6b6c6d6e6f70 # "abcdefghijklmnop"
                   7172737475767778797a             # "qrstuvwxyz"
            "#)
        }

        non_alpha(hex2value, value2hex) {
            Value::String {
                data: "\u{1F1F3}\u{1F1FF}".into(),
                bitwidth: Some(IntegerWidth::Eight),
            },
            indoc!("
                78 08               # string(8)
                   f09f87b3f09f87bf # \"\u{1F1F3}\u{1F1FF}\"
            ")
        }

        non_alpha_across_break(hex2value, value2hex) {
            Value::String {
                data: "0123456789ab\u{1F1F3}\u{1F1FF}".into(),
                bitwidth: Some(IntegerWidth::Eight),
            },
            indoc!("
                78 14                               # string(20)
                   303132333435363738396162f09f87b3 # \"0123456789ab\u{1F1F3}\"
                   f09f87bf                         # \"\u{1F1FF}\"
            ")
        }

        non_alpha_not_quite_at_break(hex2value, value2hex) {
            Value::String {
                data: "0123456789abc\u{1F1F3}\u{1F1FF}".into(),
                bitwidth: Some(IntegerWidth::Eight),
            },
            indoc!("
                78 15                               # string(21)
                   30313233343536373839616263       # \"0123456789abc\"
                   f09f87b3f09f87bf                 # \"\u{1F1F3}\u{1F1FF}\"
            ")
        }
    }

    mod u16 {
        empty(hex2value, value2hex) {
            Value::String {
                data: "".into(),
                bitwidth: Some(IntegerWidth::Sixteen),
            },
            indoc!(r#"
                79 0000 # string(0)
                        # ""
            "#)
        }

        hello(hex2value, value2hex) {
            Value::String {
                data: "hello".into(),
                bitwidth: Some(IntegerWidth::Sixteen),
            },
            indoc!(r#"
                79 0005       # string(5)
                   68656c6c6f # "hello"
            "#)
        }

        alpha(hex2value, value2hex) {
            Value::String {
                data: "abcdefghijklmnopqrstuvwxyz".into(),
                bitwidth: Some(IntegerWidth::Sixteen),
            },
            indoc!(r#"
                79 001a                             # string(26)
                   6162636465666768696a6b6c6d6e6f70 # "abcdefghijklmnop"
                   7172737475767778797a             # "qrstuvwxyz"
            "#)
        }
    }

    mod u32 {
        empty(hex2value, value2hex) {
            Value::String {
                data: "".into(),
                bitwidth: Some(IntegerWidth::ThirtyTwo),
            },
            indoc!(r#"
                7a 00000000 # string(0)
                            # ""
            "#)
        }

        hello(hex2value, value2hex) {
            Value::String {
                data: "hello".into(),
                bitwidth: Some(IntegerWidth::ThirtyTwo),
            },
            indoc!(r#"
                7a 00000005   # string(5)
                   68656c6c6f # "hello"
            "#)
        }

        alpha(hex2value, value2hex) {
            Value::String {
                data: "abcdefghijklmnopqrstuvwxyz".into(),
                bitwidth: Some(IntegerWidth::ThirtyTwo),
            },
            indoc!(r#"
                7a 0000001a                         # string(26)
                   6162636465666768696a6b6c6d6e6f70 # "abcdefghijklmnop"
                   7172737475767778797a             # "qrstuvwxyz"
            "#)
        }
    }

    mod u64 {
        empty(hex2value, value2hex) {
            Value::String {
                data: "".into(),
                bitwidth: Some(IntegerWidth::SixtyFour),
            },
            indoc!(r#"
                7b 0000000000000000 # string(0)
                                    # ""
            "#)
        }

        hello(hex2value, value2hex) {
            Value::String {
                data: "hello".into(),
                bitwidth: Some(IntegerWidth::SixtyFour),
            },
            indoc!(r#"
                7b 0000000000000005 # string(5)
                   68656c6c6f       # "hello"
            "#)
        }

        alpha(hex2value, value2hex) {
            Value::String {
                data: "abcdefghijklmnopqrstuvwxyz".into(),
                bitwidth: Some(IntegerWidth::SixtyFour),
            },
            indoc!(r#"
                7b 000000000000001a                 # string(26)
                   6162636465666768696a6b6c6d6e6f70 # "abcdefghijklmnop"
                   7172737475767778797a             # "qrstuvwxyz"
            "#)
        }
    }
}
