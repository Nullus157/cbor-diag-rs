#[macro_use]
extern crate indoc;
#[macro_use]
extern crate pretty_assertions;

extern crate cbor_diag;

use cbor_diag::{ByteString, IntegerWidth, TextString, Value};

#[macro_use]
mod utils;

testcases! {
    mod diag {
        empty(diag2value, value2diag) {
            Value::Map {
                data: vec![],
                bitwidth: Some(IntegerWidth::Unknown),
            },
            "{}",
        }

        hello_world(diag2value, value2diag) {
            Value::Map {
                data: vec![
                    (
                        Value::TextString(TextString {
                            data: "hello".into(),
                            bitwidth: IntegerWidth::Unknown,
                        }),
                        Value::TextString(TextString {
                            data: "world".into(),
                            bitwidth: IntegerWidth::Unknown,
                        }),
                    ),
                ],
                bitwidth: Some(IntegerWidth::Unknown),
            },
            r#"{ "hello": "world" }"#,
        }

        non_alpha(diag2value, value2diag) {
            Value::Map {
                data: vec![
                    (
                        Value::TextString(TextString {
                            data: "\u{1f1f3}".into(),
                            bitwidth: IntegerWidth::Unknown,
                        }),
                        Value::TextString(TextString {
                            data: "\u{1f1ff}".into(),
                            bitwidth: IntegerWidth::Unknown,
                        }),
                    ),
                ],
                bitwidth: Some(IntegerWidth::Unknown),
            },
            "{ \"\u{1f1f3}\": \"\u{1f1ff}\" }",
        }

        heterogenous(diag2value, value2diag) {
            Value::Map {
                data: vec![
                    (
                        Value::Integer {
                            value: 23,
                            bitwidth: IntegerWidth::Zero,
                        },
                        Value::TextString(TextString {
                            data: "\u{1f1f3}".into(),
                            bitwidth: IntegerWidth::Unknown,
                        }),
                    )
                ],
                bitwidth: Some(IntegerWidth::Unknown),
            },
            "{ 23: \"\u{1f1f3}\" }",
        }

        nested(diag2value, value2diag) {
            Value::Map {
                data: vec![
                    (
                        Value::Map {
                            data: vec![
                                (
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
                                    Value::ByteString(ByteString {
                                        data: "\u{1f1f3}".into(),
                                        bitwidth: IntegerWidth::Unknown,
                                    }),
                                )
                            ],
                            bitwidth: Some(IntegerWidth::Unknown),
                        },
                        Value::TextString(TextString {
                            data: "\u{1f1f3}".into(),
                            bitwidth: IntegerWidth::Unknown,
                        }),
                    ),
                    (
                        Value::Integer {
                            value: 23,
                            bitwidth: IntegerWidth::Zero,
                        },
                        Value::Map {
                            data: vec![
                                (
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
                                    Value::TextString(TextString {
                                        data: "\u{1f1f3}".into(),
                                        bitwidth: IntegerWidth::Unknown,
                                    }),
                                )
                            ],
                            bitwidth: Some(IntegerWidth::Unknown),
                        },
                    ),
                ],
                bitwidth: Some(IntegerWidth::Unknown),
            },
            "{ { [\"\u{1f1f3}\", 23]: h'f09f87b3' }: \"\u{1f1f3}\", 23: { [\"\u{1f1f3}\", 23]: \"\u{1f1f3}\" } }",
        }
    }

    mod hex {
        empty(hex2value, value2hex) {
            Value::Map {
                data: vec![],
                bitwidth: Some(IntegerWidth::Zero),
            },
            indoc!("
                a0 # map(0)
            "),
        }

        hello_world(hex2value, value2hex) {
            Value::Map {
                data: vec![
                    (
                        Value::TextString(TextString {
                            data: "hello".into(),
                            bitwidth: IntegerWidth::Zero,
                        }),
                        Value::TextString(TextString {
                            data: "world".into(),
                            bitwidth: IntegerWidth::Sixteen,
                        }),
                    ),
                ],
                bitwidth: Some(IntegerWidth::Eight),
            },
            indoc!(r#"
                b8 01            # map(1)
                   65            #   text(5)
                      68656c6c6f #     "hello"
                   79 0005       #   text(5)
                      776f726c64 #     "world"
            "#),
        }

        non_alpha(hex2value, value2hex) {
            Value::Map {
                data: vec![
                    (
                        Value::TextString(TextString {
                            data: "\u{1f1f3}".into(),
                            bitwidth: IntegerWidth::Zero,
                        }),
                        Value::TextString(TextString {
                            data: "\u{1f1ff}".into(),
                            bitwidth: IntegerWidth::Eight,
                        }),
                    ),
                ],
                bitwidth: Some(IntegerWidth::Sixteen),
            },
            indoc!("
                b9 0001        # map(1)
                   64          #   text(4)
                      f09f87b3 #     \"\u{1f1f3}\"
                   78 04       #   text(4)
                      f09f87bf #     \"\u{1f1ff}\"
            "),
        }

        heterogenous(hex2value, value2hex) {
            Value::Map {
                data: vec![
                    (
                        Value::Integer {
                            value: 23,
                            bitwidth: IntegerWidth::SixtyFour,
                        },
                        Value::TextString(TextString {
                            data: "\u{1f1f3}".into(),
                            bitwidth: IntegerWidth::Zero,
                        }),
                    )
                ],
                bitwidth: Some(IntegerWidth::Zero),
            },
            indoc!("
                a1                     # map(1)
                   1b 0000000000000017 #   unsigned(23)
                   64                  #   text(4)
                      f09f87b3         #     \"\u{1f1f3}\"
            "),
        }

        nested(hex2value, value2hex) {
            Value::Map {
                data: vec![
                    (
                        Value::Map {
                            data: vec![
                                (
                                    Value::Array {
                                        data: vec![
                                            Value::TextString(TextString {
                                                data: "\u{1f1f3}".into(),
                                                bitwidth: IntegerWidth::Zero,
                                            }),
                                            Value::Integer {
                                                value: 23,
                                                bitwidth: IntegerWidth::Zero,
                                            },
                                        ],
                                        bitwidth: Some(IntegerWidth::Zero),
                                    },
                                    Value::ByteString(ByteString {
                                        data: "\u{1f1f3}".into(),
                                        bitwidth: IntegerWidth::Zero,
                                    }),
                                )
                            ],
                            bitwidth: Some(IntegerWidth::Eight),
                        },
                        Value::TextString(TextString {
                            data: "\u{1f1f3}".into(),
                            bitwidth: IntegerWidth::Zero,
                        }),
                    ),
                    (
                        Value::Integer {
                            value: 23,
                            bitwidth: IntegerWidth::Zero,
                        },
                        Value::Map {
                            data: vec![
                                (
                                    Value::Array {
                                        data: vec![
                                            Value::TextString(TextString {
                                                data: "\u{1f1f3}".into(),
                                                bitwidth: IntegerWidth::Zero,
                                            }),
                                            Value::Integer {
                                                value: 23,
                                                bitwidth: IntegerWidth::Zero,
                                            },
                                        ],
                                        bitwidth: Some(IntegerWidth::Zero),
                                    },
                                    Value::TextString(TextString {
                                        data: "\u{1f1f3}".into(),
                                        bitwidth: IntegerWidth::Zero,
                                    }),
                                )
                            ],
                            bitwidth: Some(IntegerWidth::Zero),
                        },
                    ),
                ],
                bitwidth: Some(IntegerWidth::Sixteen),
            },
            indoc!("
                b9 0002              # map(2)
                   b8 01             #   map(1)
                      82             #     array(2)
                         64          #       text(4)
                            f09f87b3 #         \"\u{1f1f3}\"
                         17          #       unsigned(23)
                      44             #     bytes(4)
                         f09f87b3    #       \"\\xf0\\x9f\\x87\\xb3\"
                   64                #   text(4)
                      f09f87b3       #     \"\u{1f1f3}\"
                   17                #   unsigned(23)
                   a1                #   map(1)
                      82             #     array(2)
                         64          #       text(4)
                            f09f87b3 #         \"\u{1f1f3}\"
                         17          #       unsigned(23)
                      64             #     text(4)
                         f09f87b3    #       \"\u{1f1f3}\"
            "),
        }
    }

    mod indefinite {
        mod diag {
            empty(diag2value, value2diag) {
                Value::Map {
                    data: vec![],
                    bitwidth: None,
                },
                "{_ }",
            }

            hello_world(diag2value, value2diag) {
                Value::Map {
                    data: vec![
                        (
                            Value::TextString(TextString {
                                data: "hello".into(),
                                bitwidth: IntegerWidth::Unknown,
                            }),
                            Value::TextString(TextString {
                                data: "world".into(),
                                bitwidth: IntegerWidth::Unknown,
                            }),
                        ),
                    ],
                    bitwidth: None,
                },
                r#"{_ "hello": "world" }"#,
            }

            non_alpha(diag2value, value2diag) {
                Value::Map {
                    data: vec![
                        (
                            Value::TextString(TextString {
                                data: "\u{1f1f3}".into(),
                                bitwidth: IntegerWidth::Unknown,
                            }),
                            Value::TextString(TextString {
                                data: "\u{1f1ff}".into(),
                                bitwidth: IntegerWidth::Unknown,
                            }),
                        ),
                    ],
                    bitwidth: None,
                },
                "{_ \"\u{1f1f3}\": \"\u{1f1ff}\" }",
            }

            heterogenous(diag2value, value2diag) {
                Value::Map {
                    data: vec![
                        (
                            Value::Integer {
                                value: 23,
                                bitwidth: IntegerWidth::Zero,
                            },
                            Value::TextString(TextString {
                                data: "\u{1f1f3}".into(),
                                bitwidth: IntegerWidth::Unknown,
                            }),
                        )
                    ],
                    bitwidth: None,
                },
                "{_ 23: \"\u{1f1f3}\" }",
            }

            nested(diag2value, value2diag) {
                Value::Map {
                    data: vec![
                        (
                            Value::Map {
                                data: vec![
                                    (
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
                                        Value::ByteString(ByteString {
                                            data: "\u{1f1f3}".into(),
                                            bitwidth: IntegerWidth::Unknown,
                                        }),
                                    )
                                ],
                                bitwidth: Some(IntegerWidth::Unknown),
                            },
                            Value::TextString(TextString {
                                data: "\u{1f1f3}".into(),
                                bitwidth: IntegerWidth::Unknown,
                            }),
                        ),
                        (
                            Value::Integer {
                                value: 23,
                                bitwidth: IntegerWidth::Zero,
                            },
                            Value::Map {
                                data: vec![
                                    (
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
                                            bitwidth: None,
                                        },
                                        Value::TextString(TextString {
                                            data: "\u{1f1f3}".into(),
                                            bitwidth: IntegerWidth::Unknown,
                                        }),
                                    )
                                ],
                                bitwidth: None,
                            },
                        ),
                    ],
                    bitwidth: None,
                },
                "{_ { [\"\u{1f1f3}\", 23]: h'f09f87b3' }: \"\u{1f1f3}\", 23: {_ [_ \"\u{1f1f3}\", 23]: \"\u{1f1f3}\" } }",
            }
        }

        mod hex {
            empty(hex2value, value2hex) {
                Value::Map {
                    data: vec![],
                    bitwidth: None,
                },
                indoc!("
                    bf    # map(*)
                       ff #   break
                "),
            }

            hello_world(hex2value, value2hex) {
                Value::Map {
                    data: vec![
                        (
                            Value::TextString(TextString {
                                data: "hello".into(),
                                bitwidth: IntegerWidth::Zero,
                            }),
                            Value::TextString(TextString {
                                data: "world".into(),
                                bitwidth: IntegerWidth::Sixteen,
                            }),
                        ),
                    ],
                    bitwidth: None,
                },
                indoc!(r#"
                    bf               # map(*)
                       65            #   text(5)
                          68656c6c6f #     "hello"
                       79 0005       #   text(5)
                          776f726c64 #     "world"
                       ff            #   break
                "#),
            }

            non_alpha(hex2value, value2hex) {
                Value::Map {
                    data: vec![
                        (
                            Value::TextString(TextString {
                                data: "\u{1f1f3}".into(),
                                bitwidth: IntegerWidth::Zero,
                            }),
                            Value::TextString(TextString {
                                data: "\u{1f1ff}".into(),
                                bitwidth: IntegerWidth::Eight,
                            }),
                        ),
                    ],
                    bitwidth: None,
                },
                indoc!("
                    bf             # map(*)
                       64          #   text(4)
                          f09f87b3 #     \"\u{1f1f3}\"
                       78 04       #   text(4)
                          f09f87bf #     \"\u{1f1ff}\"
                       ff          #   break
                "),
            }

            heterogenous(hex2value, value2hex) {
                Value::Map {
                    data: vec![
                        (
                            Value::Integer {
                                value: 23,
                                bitwidth: IntegerWidth::SixtyFour,
                            },
                            Value::TextString(TextString {
                                data: "\u{1f1f3}".into(),
                                bitwidth: IntegerWidth::Zero,
                            }),
                        )
                    ],
                    bitwidth: None,
                },
                indoc!("
                    bf                     # map(*)
                       1b 0000000000000017 #   unsigned(23)
                       64                  #   text(4)
                          f09f87b3         #     \"\u{1f1f3}\"
                       ff                  #   break
                "),
            }

            nested(hex2value, value2hex) {
                Value::Map {
                    data: vec![
                        (
                            Value::Map {
                                data: vec![
                                    (
                                        Value::Array {
                                            data: vec![
                                                Value::TextString(TextString {
                                                    data: "\u{1f1f3}".into(),
                                                    bitwidth: IntegerWidth::Zero,
                                                }),
                                                Value::Integer {
                                                    value: 23,
                                                    bitwidth: IntegerWidth::Zero,
                                                },
                                            ],
                                            bitwidth: Some(IntegerWidth::Zero),
                                        },
                                        Value::ByteString(ByteString {
                                            data: "\u{1f1f3}".into(),
                                            bitwidth: IntegerWidth::Zero,
                                        }),
                                    )
                                ],
                                bitwidth: Some(IntegerWidth::Eight),
                            },
                            Value::TextString(TextString {
                                data: "\u{1f1f3}".into(),
                                bitwidth: IntegerWidth::Zero,
                            }),
                        ),
                        (
                            Value::Integer {
                                value: 23,
                                bitwidth: IntegerWidth::Zero,
                            },
                            Value::Map {
                                data: vec![
                                    (
                                        Value::Array {
                                            data: vec![
                                                Value::TextString(TextString {
                                                    data: "\u{1f1f3}".into(),
                                                    bitwidth: IntegerWidth::Zero,
                                                }),
                                                Value::Integer {
                                                    value: 23,
                                                    bitwidth: IntegerWidth::Zero,
                                                },
                                            ],
                                            bitwidth: None,
                                        },
                                        Value::TextString(TextString {
                                            data: "\u{1f1f3}".into(),
                                            bitwidth: IntegerWidth::Zero,
                                        }),
                                    )
                                ],
                                bitwidth: None,
                            },
                        ),
                    ],
                    bitwidth: None,
                },
                indoc!("
                    bf                   # map(*)
                       b8 01             #   map(1)
                          82             #     array(2)
                             64          #       text(4)
                                f09f87b3 #         \"\u{1f1f3}\"
                             17          #       unsigned(23)
                          44             #     bytes(4)
                             f09f87b3    #       \"\\xf0\\x9f\\x87\\xb3\"
                       64                #   text(4)
                          f09f87b3       #     \"\u{1f1f3}\"
                       17                #   unsigned(23)
                       bf                #   map(*)
                          9f             #     array(*)
                             64          #       text(4)
                                f09f87b3 #         \"\u{1f1f3}\"
                             17          #       unsigned(23)
                             ff          #       break
                          64             #     text(4)
                             f09f87b3    #       \"\u{1f1f3}\"
                          ff             #     break
                       ff                #   break
                "),
            }
        }
    }
}
