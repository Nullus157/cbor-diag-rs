use cbor_diag::{ByteString, DataItem, IntegerWidth, TextString};
use indoc::indoc;

#[macro_use]
mod utils;

testcases! {
    mod diag {
        empty(diag2value, value2diag) {
            DataItem::Map {
                data: vec![],
                bitwidth: Some(IntegerWidth::Unknown),
            },
            {
                "{}",
                "{}",
            }
        }

        hello_world(diag2value, value2diag) {
            DataItem::Map {
                data: vec![
                    (
                        DataItem::TextString(TextString {
                            data: "hello".into(),
                            bitwidth: IntegerWidth::Unknown,
                        }),
                        DataItem::TextString(TextString {
                            data: "world".into(),
                            bitwidth: IntegerWidth::Unknown,
                        }),
                    ),
                ],
                bitwidth: Some(IntegerWidth::Unknown),
            },
            {
                r#"{"hello":"world"}"#,
                r#"{"hello": "world"}"#,
            }
        }

        hello_world_trailingcomma(diag2value) {
            DataItem::Map {
                data: vec![
                    (
                        DataItem::TextString(TextString {
                            data: "hello".into(),
                            bitwidth: IntegerWidth::Unknown,
                        }),
                        DataItem::TextString(TextString {
                            data: "world".into(),
                            bitwidth: IntegerWidth::Unknown,
                        }),
                    ),
                ],
                bitwidth: Some(IntegerWidth::Unknown),
            },
            {
                r#"{"hello":"world",}"#,
                r#"{"hello": "world",}"#,
            }
        }

        non_alpha(diag2value, value2diag) {
            DataItem::Map {
                data: vec![
                    (
                        DataItem::TextString(TextString {
                            data: "\u{1f1f3}".into(),
                            bitwidth: IntegerWidth::Unknown,
                        }),
                        DataItem::TextString(TextString {
                            data: "\u{1f1ff}".into(),
                            bitwidth: IntegerWidth::Unknown,
                        }),
                    ),
                ],
                bitwidth: Some(IntegerWidth::Unknown),
            },
            {
                "{\"\u{1f1f3}\":\"\u{1f1ff}\"}",
                "{\"\u{1f1f3}\": \"\u{1f1ff}\"}",
            }
        }

        heterogenous(diag2value, value2diag) {
            DataItem::Map {
                data: vec![
                    (
                        DataItem::Integer {
                            value: 23,
                            bitwidth: IntegerWidth::Zero,
                        },
                        DataItem::TextString(TextString {
                            data: "\u{1f1f3}".into(),
                            bitwidth: IntegerWidth::Unknown,
                        }),
                    )
                ],
                bitwidth: Some(IntegerWidth::Unknown),
            },
            {
                "{23:\"\u{1f1f3}\"}",
                "{23: \"\u{1f1f3}\"}",
            }
        }

        nested(diag2value, value2diag) {
            DataItem::Map {
                data: vec![
                    (
                        DataItem::Map {
                            data: vec![
                                (
                                    DataItem::Array {
                                        data: vec![
                                            DataItem::TextString(TextString {
                                                data: "\u{1f1f3}".into(),
                                                bitwidth: IntegerWidth::Unknown,
                                            }),
                                            DataItem::Integer {
                                                value: 23,
                                                bitwidth: IntegerWidth::Zero,
                                            },
                                        ],
                                        bitwidth: Some(IntegerWidth::Unknown),
                                    },
                                    DataItem::ByteString(ByteString {
                                        data: "\u{1f1f3}".into(),
                                        bitwidth: IntegerWidth::Unknown,
                                    }),
                                )
                            ],
                            bitwidth: Some(IntegerWidth::Unknown),
                        },
                        DataItem::TextString(TextString {
                            data: "\u{1f1f3}".into(),
                            bitwidth: IntegerWidth::Unknown,
                        }),
                    ),
                    (
                        DataItem::Integer {
                            value: 23,
                            bitwidth: IntegerWidth::Zero,
                        },
                        DataItem::Map {
                            data: vec![
                                (
                                    DataItem::Array {
                                        data: vec![
                                            DataItem::TextString(TextString {
                                                data: "\u{1f1f3}".into(),
                                                bitwidth: IntegerWidth::Unknown,
                                            }),
                                            DataItem::Integer {
                                                value: 23,
                                                bitwidth: IntegerWidth::Zero,
                                            },
                                        ],
                                        bitwidth: Some(IntegerWidth::Unknown),
                                    },
                                    DataItem::TextString(TextString {
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
            {
                "{{[\"\u{1f1f3}\",23]:h'f09f87b3'}:\"\u{1f1f3}\",23:{[\"\u{1f1f3}\",23]:\"\u{1f1f3}\"}}",
                "
                {
                    {[\"\u{1f1f3}\", 23]: h'f09f87b3'}: \"\u{1f1f3}\",
                    23: {[\"\u{1f1f3}\", 23]: \"\u{1f1f3}\"},
                }
                ",
            }
        }
    }

    mod hex {
        empty(hex2value, value2hex) {
            DataItem::Map {
                data: vec![],
                bitwidth: Some(IntegerWidth::Zero),
            },
            indoc!("
                a0 # map(0)
            "),
        }

        hello_world(hex2value, value2hex) {
            DataItem::Map {
                data: vec![
                    (
                        DataItem::TextString(TextString {
                            data: "hello".into(),
                            bitwidth: IntegerWidth::Zero,
                        }),
                        DataItem::TextString(TextString {
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
            DataItem::Map {
                data: vec![
                    (
                        DataItem::TextString(TextString {
                            data: "\u{1f1f3}".into(),
                            bitwidth: IntegerWidth::Zero,
                        }),
                        DataItem::TextString(TextString {
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
            DataItem::Map {
                data: vec![
                    (
                        DataItem::Integer {
                            value: 23,
                            bitwidth: IntegerWidth::SixtyFour,
                        },
                        DataItem::TextString(TextString {
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
            DataItem::Map {
                data: vec![
                    (
                        DataItem::Map {
                            data: vec![
                                (
                                    DataItem::Array {
                                        data: vec![
                                            DataItem::TextString(TextString {
                                                data: "\u{1f1f3}".into(),
                                                bitwidth: IntegerWidth::Zero,
                                            }),
                                            DataItem::Integer {
                                                value: 23,
                                                bitwidth: IntegerWidth::Zero,
                                            },
                                        ],
                                        bitwidth: Some(IntegerWidth::Zero),
                                    },
                                    DataItem::ByteString(ByteString {
                                        data: "\u{1f1f3}".into(),
                                        bitwidth: IntegerWidth::Zero,
                                    }),
                                )
                            ],
                            bitwidth: Some(IntegerWidth::Eight),
                        },
                        DataItem::TextString(TextString {
                            data: "\u{1f1f3}".into(),
                            bitwidth: IntegerWidth::Zero,
                        }),
                    ),
                    (
                        DataItem::Integer {
                            value: 23,
                            bitwidth: IntegerWidth::Zero,
                        },
                        DataItem::Map {
                            data: vec![
                                (
                                    DataItem::Array {
                                        data: vec![
                                            DataItem::TextString(TextString {
                                                data: "\u{1f1f3}".into(),
                                                bitwidth: IntegerWidth::Zero,
                                            }),
                                            DataItem::Integer {
                                                value: 23,
                                                bitwidth: IntegerWidth::Zero,
                                            },
                                        ],
                                        bitwidth: Some(IntegerWidth::Zero),
                                    },
                                    DataItem::TextString(TextString {
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
                DataItem::Map {
                    data: vec![],
                    bitwidth: None,
                },
                {
                    "{_}",
                    "{_ }",
                }
            }

            hello_world(diag2value, value2diag) {
                DataItem::Map {
                    data: vec![
                        (
                            DataItem::TextString(TextString {
                                data: "hello".into(),
                                bitwidth: IntegerWidth::Unknown,
                            }),
                            DataItem::TextString(TextString {
                                data: "world".into(),
                                bitwidth: IntegerWidth::Unknown,
                            }),
                        ),
                    ],
                    bitwidth: None,
                },
                {
                    r#"{_"hello":"world"}"#,
                    r#"{_ "hello": "world"}"#,
                }
            }

            non_alpha(diag2value, value2diag) {
                DataItem::Map {
                    data: vec![
                        (
                            DataItem::TextString(TextString {
                                data: "\u{1f1f3}".into(),
                                bitwidth: IntegerWidth::Unknown,
                            }),
                            DataItem::TextString(TextString {
                                data: "\u{1f1ff}".into(),
                                bitwidth: IntegerWidth::Unknown,
                            }),
                        ),
                    ],
                    bitwidth: None,
                },
                {
                    "{_\"\u{1f1f3}\":\"\u{1f1ff}\"}",
                    "{_ \"\u{1f1f3}\": \"\u{1f1ff}\"}",
                }
            }

            heterogenous(diag2value, value2diag) {
                DataItem::Map {
                    data: vec![
                        (
                            DataItem::Integer {
                                value: 23,
                                bitwidth: IntegerWidth::Zero,
                            },
                            DataItem::TextString(TextString {
                                data: "\u{1f1f3}".into(),
                                bitwidth: IntegerWidth::Unknown,
                            }),
                        )
                    ],
                    bitwidth: None,
                },
                {
                    "{_23:\"\u{1f1f3}\"}",
                    "{_ 23: \"\u{1f1f3}\"}",
                }
            }

            nested(diag2value, value2diag) {
                DataItem::Map {
                    data: vec![
                        (
                            DataItem::Map {
                                data: vec![
                                    (
                                        DataItem::Array {
                                            data: vec![
                                                DataItem::TextString(TextString {
                                                    data: "\u{1f1f3}".into(),
                                                    bitwidth: IntegerWidth::Unknown,
                                                }),
                                                DataItem::Integer {
                                                    value: 23,
                                                    bitwidth: IntegerWidth::Zero,
                                                },
                                            ],
                                            bitwidth: Some(IntegerWidth::Unknown),
                                        },
                                        DataItem::ByteString(ByteString {
                                            data: "\u{1f1f3}".into(),
                                            bitwidth: IntegerWidth::Unknown,
                                        }),
                                    )
                                ],
                                bitwidth: Some(IntegerWidth::Unknown),
                            },
                            DataItem::TextString(TextString {
                                data: "\u{1f1f3}".into(),
                                bitwidth: IntegerWidth::Unknown,
                            }),
                        ),
                        (
                            DataItem::Integer {
                                value: 23,
                                bitwidth: IntegerWidth::Zero,
                            },
                            DataItem::Map {
                                data: vec![
                                    (
                                        DataItem::Array {
                                            data: vec![
                                                DataItem::TextString(TextString {
                                                    data: "\u{1f1f3}".into(),
                                                    bitwidth: IntegerWidth::Unknown,
                                                }),
                                                DataItem::Integer {
                                                    value: 23,
                                                    bitwidth: IntegerWidth::Zero,
                                                },
                                            ],
                                            bitwidth: None,
                                        },
                                        DataItem::TextString(TextString {
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
                {
                    "{_{[\"\u{1f1f3}\",23]:h'f09f87b3'}:\"\u{1f1f3}\",23:{_[_\"\u{1f1f3}\",23]:\"\u{1f1f3}\"}}",
                    "
                    {_
                        {[\"\u{1f1f3}\", 23]: h'f09f87b3'}: \"\u{1f1f3}\",
                        23: {_ [_ \"\u{1f1f3}\", 23]: \"\u{1f1f3}\"},
                    }
                    ",
                }
            }
        }

        mod hex {
            empty(hex2value, value2hex) {
                DataItem::Map {
                    data: vec![],
                    bitwidth: None,
                },
                indoc!("
                    bf    # map(*)
                       ff #   break
                "),
            }

            hello_world(hex2value, value2hex) {
                DataItem::Map {
                    data: vec![
                        (
                            DataItem::TextString(TextString {
                                data: "hello".into(),
                                bitwidth: IntegerWidth::Zero,
                            }),
                            DataItem::TextString(TextString {
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
                DataItem::Map {
                    data: vec![
                        (
                            DataItem::TextString(TextString {
                                data: "\u{1f1f3}".into(),
                                bitwidth: IntegerWidth::Zero,
                            }),
                            DataItem::TextString(TextString {
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
                DataItem::Map {
                    data: vec![
                        (
                            DataItem::Integer {
                                value: 23,
                                bitwidth: IntegerWidth::SixtyFour,
                            },
                            DataItem::TextString(TextString {
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
                DataItem::Map {
                    data: vec![
                        (
                            DataItem::Map {
                                data: vec![
                                    (
                                        DataItem::Array {
                                            data: vec![
                                                DataItem::TextString(TextString {
                                                    data: "\u{1f1f3}".into(),
                                                    bitwidth: IntegerWidth::Zero,
                                                }),
                                                DataItem::Integer {
                                                    value: 23,
                                                    bitwidth: IntegerWidth::Zero,
                                                },
                                            ],
                                            bitwidth: Some(IntegerWidth::Zero),
                                        },
                                        DataItem::ByteString(ByteString {
                                            data: "\u{1f1f3}".into(),
                                            bitwidth: IntegerWidth::Zero,
                                        }),
                                    )
                                ],
                                bitwidth: Some(IntegerWidth::Eight),
                            },
                            DataItem::TextString(TextString {
                                data: "\u{1f1f3}".into(),
                                bitwidth: IntegerWidth::Zero,
                            }),
                        ),
                        (
                            DataItem::Integer {
                                value: 23,
                                bitwidth: IntegerWidth::Zero,
                            },
                            DataItem::Map {
                                data: vec![
                                    (
                                        DataItem::Array {
                                            data: vec![
                                                DataItem::TextString(TextString {
                                                    data: "\u{1f1f3}".into(),
                                                    bitwidth: IntegerWidth::Zero,
                                                }),
                                                DataItem::Integer {
                                                    value: 23,
                                                    bitwidth: IntegerWidth::Zero,
                                                },
                                            ],
                                            bitwidth: None,
                                        },
                                        DataItem::TextString(TextString {
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
