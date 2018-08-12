#[macro_use]
extern crate indoc;
#[macro_use]
extern crate pretty_assertions;
extern crate hex;

extern crate cbor_diag;

use cbor_diag::{ByteString, FloatWidth, IntegerWidth, Tag, TextString, Value};

#[macro_use]
mod utils;

testcases! {
    mod diag {
        date_time(diag2value, value2diag) {
            Value::Array {
                data: vec![
                    Value::Tag {
                        tag: Tag::DATETIME,
                        bitwidth: IntegerWidth::Zero,
                        value: Box::new(Value::TextString(TextString {
                            data: "2018-08-02T18:19:38Z".into(),
                            bitwidth: IntegerWidth::Unknown,
                        }))
                    },
                    Value::Tag {
                        tag: Tag::DATETIME,
                        bitwidth: IntegerWidth::Zero,
                        value: Box::new(Value::TextString(TextString {
                            data: "1921-06-01T05:40:21Z".into(),
                            bitwidth: IntegerWidth::Unknown,
                        }))
                    },
                    Value::Tag {
                        tag: Tag::DATETIME,
                        bitwidth: IntegerWidth::Zero,
                        value: Box::new(Value::TextString(TextString {
                            data: "2018-08-02T18:19:38.125Z".into(),
                            bitwidth: IntegerWidth::Unknown,
                        }))
                    },
                ],
                bitwidth: Some(IntegerWidth::Unknown),
            },
            r#"[0("2018-08-02T18:19:38Z"), 0("1921-06-01T05:40:21Z"), 0("2018-08-02T18:19:38.125Z")]"#,
        }

        epoch_date_time(diag2value, value2diag) {
            Value::Array {
                data: vec![
                    Value::Tag {
                        tag: Tag::EPOCH_DATETIME,
                        bitwidth: IntegerWidth::Zero,
                        value: Box::new(Value::Integer {
                            value: 1533233978,
                            bitwidth: IntegerWidth::Unknown,
                        })
                    },
                    Value::Tag {
                        tag: Tag::EPOCH_DATETIME,
                        bitwidth: IntegerWidth::Zero,
                        value: Box::new(Value::Negative {
                            value: 1533233978,
                            bitwidth: IntegerWidth::Unknown,
                        })
                    },
                    Value::Tag {
                        tag: Tag::EPOCH_DATETIME,
                        bitwidth: IntegerWidth::Zero,
                        value: Box::new(Value::Float {
                            value: 1533233978.125,
                            bitwidth: FloatWidth::Unknown,
                        })
                    },
                ],
                bitwidth: Some(IntegerWidth::Unknown),
            },
            r#"[1(1533233978), 1(-1533233979), 1(1533233978.125)]"#,
        }

        positive_bignum(diag2value, value2diag) {
            Value::Tag {
                tag: Tag::POSITIVE_BIGNUM,
                bitwidth: IntegerWidth::Zero,
                value: Box::new(Value::ByteString(ByteString {
                    data: hex::decode(
                        "000001ffffffffffffffffffffff0000000000000000000000"
                    ).unwrap(),
                    bitwidth: IntegerWidth::Unknown,
                }))
            },
            "2(h'000001ffffffffffffffffffffff0000000000000000000000')",
        }

        negative_bignum(diag2value, value2diag) {
            Value::Tag {
                tag: Tag::NEGATIVE_BIGNUM,
                bitwidth: IntegerWidth::Zero,
                value: Box::new(Value::ByteString(ByteString {
                    data: hex::decode(
                        "123456789abcdeffedcba987654321"
                    ).unwrap(),
                    bitwidth: IntegerWidth::Unknown,
                }))
            },
            "3(h'123456789abcdeffedcba987654321')",
        }
    }

    mod hex_tests {
        date_time(hex2value, value2hex) {
            Value::Array {
                data: vec![
                    Value::Tag {
                        tag: Tag::DATETIME,
                        bitwidth: IntegerWidth::Zero,
                        value: Box::new(Value::TextString(TextString {
                            data: "2018-08-02T18:19:38Z".into(),
                            bitwidth: IntegerWidth::Zero,
                        }))
                    },
                    Value::Tag {
                        tag: Tag::DATETIME,
                        bitwidth: IntegerWidth::Zero,
                        value: Box::new(Value::TextString(TextString {
                            data: "1921-06-01T05:40:21Z".into(),
                            bitwidth: IntegerWidth::Zero,
                        }))
                    },
                    Value::Tag {
                        tag: Tag::DATETIME,
                        bitwidth: IntegerWidth::Zero,
                        value: Box::new(Value::TextString(TextString {
                            data: "2018-08-02T18:19:38.125Z".into(),
                            bitwidth: IntegerWidth::Eight,
                        }))
                    },
                ],
                bitwidth: Some(IntegerWidth::Zero),
            },
            indoc!(r#"
               83                                                        # array(3)
                  c0                                                     #   standard datetime string, tag(0)
                     74                                                  #     text(20)
                        323031382d30382d30325431383a31393a33385a         #       "2018-08-02T18:19:38Z"
                                                                         #     epoch(1533233978)
                  c0                                                     #   standard datetime string, tag(0)
                     74                                                  #     text(20)
                        313932312d30362d30315430353a34303a32315a         #       "1921-06-01T05:40:21Z"
                                                                         #     epoch(-1533233979)
                  c0                                                     #   standard datetime string, tag(0)
                     78 18                                               #     text(24)
                        323031382d30382d30325431383a31393a33382e3132355a #       "2018-08-02T18:19:38.125Z"
                                                                         #     epoch(1533233978.125)
            "#),
        }

        epoch_date_time(hex2value, value2hex) {
            Value::Array {
                data: vec![
                    Value::Tag {
                        tag: Tag::EPOCH_DATETIME,
                        bitwidth: IntegerWidth::Zero,
                        value: Box::new(Value::Integer {
                            value: 1533233978,
                            bitwidth: IntegerWidth::ThirtyTwo,
                        })
                    },
                    Value::Tag {
                        tag: Tag::EPOCH_DATETIME,
                        bitwidth: IntegerWidth::Zero,
                        value: Box::new(Value::Negative {
                            value: 1533233978,
                            bitwidth: IntegerWidth::ThirtyTwo,
                        })
                    },
                    Value::Tag {
                        tag: Tag::EPOCH_DATETIME,
                        bitwidth: IntegerWidth::Zero,
                        value: Box::new(Value::Float {
                            value: 1533233978.125,
                            bitwidth: FloatWidth::SixtyFour,
                        })
                    },
                ],
                bitwidth: Some(IntegerWidth::Zero),
            },
            indoc!(r#"
                83                        # array(3)
                   c1                     #   epoch datetime value, tag(1)
                      1a 5b634b3a         #     unsigned(1533233978)
                                          #     datetime(2018-08-02T18:19:38Z)
                   c1                     #   epoch datetime value, tag(1)
                      3a 5b634b3a         #     negative(1533233978)
                                          #     datetime(1921-06-01T05:40:21Z)
                   c1                     #   epoch datetime value, tag(1)
                      fb 41d6d8d2ce880000 #     float(1533233978.125)
                                          #     datetime(2018-08-02T18:19:38.125Z)
            "#),
        }

        positive_bignum(hex2value, value2hex) {
            Value::Tag {
                tag: Tag::POSITIVE_BIGNUM,
                bitwidth: IntegerWidth::Zero,
                value: Box::new(Value::ByteString(ByteString {
                    data: hex::decode(
                        "000001ffffffffffffffffffffff0000000000000000000000"
                    ).unwrap(),
                    bitwidth: IntegerWidth::Eight,
                }))
            },
            indoc!(r#"
                c2                                     # positive bignum, tag(2)
                   58 19                               #   bytes(25)
                      000001ffffffffffffffffffffff0000 #     "\x00\x00\x01\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\x00\x00"
                      000000000000000000               #     "\x00\x00\x00\x00\x00\x00\x00\x00\x00"
                                                       #   bignum(191561942608236107294793378084303638130997321548169216)
            "#),
        }

        negative_bignum(hex2value, value2hex) {
            Value::Tag {
                tag: Tag::NEGATIVE_BIGNUM,
                bitwidth: IntegerWidth::Zero,
                value: Box::new(Value::ByteString(ByteString {
                    data: hex::decode(
                        "123456789abcdeffedcba987654321"
                    ).unwrap(),
                    bitwidth: IntegerWidth::Eight,
                }))
            },
            indoc!(r#"
                c3                                   # negative bignum, tag(3)
                   58 0f                             #   bytes(15)
                      123456789abcdeffedcba987654321 #     "\x124Vx\x9a\xbc\xde\xff\xed\xcb\xa9\x87eC!"
                                                     #   bignum(-94522879700260684208272210605196066)
            "#),
        }
    }
}
