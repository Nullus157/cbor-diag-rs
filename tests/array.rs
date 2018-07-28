#[macro_use]
extern crate indoc;
#[macro_use]
extern crate pretty_assertions;

extern crate cbor_diag;

use cbor_diag::{IntegerWidth, TextString, Value};

#[macro_use]
mod utils;

// CBOR diagnostic notation provides for no way to encode the width of the
// length value of an array, so unfortunately roundtripping cannot be
// supported.
//
// Maybe I can just extend diagnostic notation for this?

testcases! {
    mod diag {
        empty(diag2value, value2diag) {
            Value::Array {
                data: vec![],
                bitwidth: Some(IntegerWidth::Unknown),
            },
            "[]",
        }

        hello_world(diag2value, value2diag) {
            Value::Array {
                data: vec![
                    Value::TextString(TextString {
                        data: "hello".into(),
                        bitwidth: IntegerWidth::Unknown,
                    }),
                    Value::TextString(TextString {
                        data: "world".into(),
                        bitwidth: IntegerWidth::Unknown,
                    }),
                ],
                bitwidth: Some(IntegerWidth::Unknown),
            },
            r#"["hello", "world"]"#,
        }

        non_alpha(diag2value, value2diag) {
            Value::Array {
                data: vec![
                    Value::TextString(TextString {
                        data: "\u{1f1f3}".into(),
                        bitwidth: IntegerWidth::Unknown,
                    }),
                    Value::TextString(TextString {
                        data: "\u{1f1ff}".into(),
                        bitwidth: IntegerWidth::Unknown,
                    }),
                ],
                bitwidth: Some(IntegerWidth::Unknown),
            },
            "[\"\u{1f1f3}\", \"\u{1f1ff}\"]",
        }

        heterogenous(diag2value, value2diag) {
            Value::Array {
                data: vec![
                    Value::TextString(TextString {
                        data: "\u{1f1f3}".into(),
                        bitwidth: IntegerWidth::Unknown,
                    }),
                    Value::Integer {
                        value: 23,
                        bitwidth: IntegerWidth::Zero,
                    },
                ],
                bitwidth: Some(IntegerWidth::Unknown),
            },
            "[\"\u{1f1f3}\", 23]",
        }

        nested(diag2value, value2diag) {
            Value::Array {
                data: vec![
                    Value::TextString(TextString {
                        data: "\u{1f1f3}".into(),
                        bitwidth: IntegerWidth::Unknown,
                    }),
                    Value::Integer {
                        value: 23,
                        bitwidth: IntegerWidth::Zero,
                    },
                    Value::Array {
                        data: vec![
                            Value::TextString(TextString {
                                data: "\u{1f1f3}".into(),
                                bitwidth: IntegerWidth::Unknown,
                            }),
                            Value::Integer {
                                value: 23,
                                bitwidth: IntegerWidth::Zero,
                            },
                        ],
                        bitwidth: Some(IntegerWidth::Unknown),
                    },
                ],
                bitwidth: Some(IntegerWidth::Unknown),
            },
            "[\"\u{1f1f3}\", 23, [\"\u{1f1f3}\", 23]]",
        }
    }

    mod hex {
        empty(hex2value, value2hex) {
            Value::Array {
                data: vec![],
                bitwidth: Some(IntegerWidth::Zero),
            },
            indoc!("
                80 # array(0)
            "),
        }

        hello_world(hex2value, value2hex) {
            Value::Array {
                data: vec![
                    Value::TextString(TextString {
                        data: "hello".into(),
                        bitwidth: IntegerWidth::Zero,
                    }),
                    Value::TextString(TextString {
                        data: "world".into(),
                        bitwidth: IntegerWidth::Sixteen,
                    }),
                ],
                bitwidth: Some(IntegerWidth::Eight),
            },
            indoc!(r#"
                98 02            # array(2)
                   65            #   text(5)
                      68656c6c6f #     "hello"
                   79 0005       #   text(5)
                      776f726c64 #     "world"
            "#),
        }

        non_alpha(hex2value, value2hex) {
            Value::Array {
                data: vec![
                    Value::TextString(TextString {
                        data: "\u{1f1f3}".into(),
                        bitwidth: IntegerWidth::Zero,
                    }),
                    Value::TextString(TextString {
                        data: "\u{1f1ff}".into(),
                        bitwidth: IntegerWidth::Eight,
                    }),
                ],
                bitwidth: Some(IntegerWidth::Sixteen),
            },
            indoc!("
                99 0002        # array(2)
                   64          #   text(4)
                      f09f87b3 #     \"\u{1f1f3}\"
                   78 04       #   text(4)
                      f09f87bf #     \"\u{1f1ff}\"
            "),
        }

        heterogenous(hex2value, value2hex) {
            Value::Array {
                data: vec![
                    Value::TextString(TextString {
                        data: "\u{1f1f3}".into(),
                        bitwidth: IntegerWidth::Zero,
                    }),
                    Value::Integer {
                        value: 23,
                        bitwidth: IntegerWidth::SixtyFour,
                    },
                ],
                bitwidth: Some(IntegerWidth::Zero),
            },
            indoc!("
                82                     # array(2)
                   64                  #   text(4)
                      f09f87b3         #     \"\u{1f1f3}\"
                   1b 0000000000000017 #   unsigned(23)
            "),
        }

        nested(hex2value, value2hex) {
            Value::Array {
                data: vec![
                    Value::TextString(TextString {
                        data: "\u{1f1f3}".into(),
                        bitwidth: IntegerWidth::Zero,
                    }),
                    Value::Integer {
                        value: 23,
                        bitwidth: IntegerWidth::SixtyFour,
                    },
                    Value::Array {
                        data: vec![
                            Value::TextString(TextString {
                                data: "\u{1f1f3}".into(),
                                bitwidth: IntegerWidth::Zero,
                            }),
                            Value::Integer {
                                value: 23,
                                bitwidth: IntegerWidth::SixtyFour,
                            },
                        ],
                        bitwidth: Some(IntegerWidth::ThirtyTwo),
                    },
                ],
                bitwidth: Some(IntegerWidth::Zero),
            },
            indoc!("
                83                        # array(3)
                   64                     #   text(4)
                      f09f87b3            #     \"\u{1f1f3}\"
                   1b 0000000000000017    #   unsigned(23)
                   9a 00000002            #   array(2)
                      64                  #     text(4)
                         f09f87b3         #       \"\u{1f1f3}\"
                      1b 0000000000000017 #     unsigned(23)
            "),
        }
    }
}
