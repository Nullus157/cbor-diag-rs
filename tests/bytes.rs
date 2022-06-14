use cbor_diag::{ByteString, DataItem, IntegerWidth};
use data_encoding_macro::hexlower as hex;
use indoc::indoc;

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
            DataItem::ByteString(ByteString {
                data: vec![],
                bitwidth: IntegerWidth::Unknown,
            }),
            {
                "h''",
                "h''",
            }
        }

        hello(diag2value, value2diag) {
            DataItem::ByteString(ByteString {
                data: b"hello"[..].into(),
                bitwidth: IntegerWidth::Unknown,
            }),
            {
                "h'68656c6c6f'",
                "h'68656c6c6f'",
            }
        }

        alpha(diag2value, value2diag) {
            DataItem::ByteString(ByteString {
                data: b"abcdefghijklmnopqrstuvwxyz"[..].into(),
                bitwidth: IntegerWidth::Unknown,
            }),
            {
                "h'6162636465666768696a6b6c6d6e6f707172737475767778797a'",
                "h'6162636465666768696a6b6c6d6e6f707172737475767778797a'",
            }
        }

        non_alpha(diag2value, value2diag) {
            DataItem::ByteString(ByteString {
                data: vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
                bitwidth: IntegerWidth::Unknown,
            }),
            {
                "h'000102030405060708090a'",
                "h'000102030405060708090a'",
            }
        }
    }

    mod tiny {
        empty(hex2value, value2hex) {
            DataItem::ByteString(ByteString {
                data: vec![],
                bitwidth: IntegerWidth::Zero,
            }),
            indoc!(r#"
                40 # bytes(0)
                   #   ""
            "#)
        }

        hello(hex2value, value2hex) {
            DataItem::ByteString(ByteString {
                data: b"hello"[..].into(),
                bitwidth: IntegerWidth::Zero,
            }),
            indoc!(r#"
                45            # bytes(5)
                   68656c6c6f #   "hello"
            "#)
        }
    }

    mod u8 {
        empty(hex2value, value2hex) {
            DataItem::ByteString(ByteString {
                data: vec![],
                bitwidth: IntegerWidth::Eight,
            }),
            indoc!(r#"
                58 00 # bytes(0)
                      #   ""
            "#)
        }

        hello(hex2value, value2hex) {
            DataItem::ByteString(ByteString {
                data: b"hello"[..].into(),
                bitwidth: IntegerWidth::Eight,
            }),
            indoc!(r#"
                58 05         # bytes(5)
                   68656c6c6f #   "hello"
            "#)
        }

        alpha(hex2value, value2hex) {
            DataItem::ByteString(ByteString {
                data: b"abcdefghijklmnopqrstuvwxyz"[..].into(),
                bitwidth: IntegerWidth::Eight,
            }),
            indoc!(r#"
                58 1a                               # bytes(26)
                   6162636465666768696a6b6c6d6e6f70 #   "abcdefghijklmnop"
                   7172737475767778797a             #   "qrstuvwxyz"
            "#)
        }

        non_alpha(hex2value, value2hex) {
            DataItem::ByteString(ByteString {
                data: vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
                bitwidth: IntegerWidth::Eight,
            }),
            indoc!(r#"
                58 0b                     # bytes(11)
                   000102030405060708090a #   "\x00\x01\x02\x03\x04\x05\x06\x07\x08\t\n"
            "#)
        }
    }

    mod u16 {
        empty(hex2value, value2hex) {
            DataItem::ByteString(ByteString {
                data: vec![],
                bitwidth: IntegerWidth::Sixteen,
            }),
            indoc!(r#"
                59 0000 # bytes(0)
                        #   ""
            "#)
        }

        hello(hex2value, value2hex) {
            DataItem::ByteString(ByteString {
                data: b"hello"[..].into(),
                bitwidth: IntegerWidth::Sixteen,
            }),
            indoc!(r#"
                59 0005       # bytes(5)
                   68656c6c6f #   "hello"
            "#)
        }

        alpha(hex2value, value2hex) {
            DataItem::ByteString(ByteString {
                data: b"abcdefghijklmnopqrstuvwxyz"[..].into(),
                bitwidth: IntegerWidth::Sixteen,
            }),
            indoc!(r#"
                59 001a                             # bytes(26)
                   6162636465666768696a6b6c6d6e6f70 #   "abcdefghijklmnop"
                   7172737475767778797a             #   "qrstuvwxyz"
            "#)
        }
    }

    mod u32 {
        empty(hex2value, value2hex) {
            DataItem::ByteString(ByteString {
                data: vec![],
                bitwidth: IntegerWidth::ThirtyTwo,
            }),
            indoc!(r#"
                5a 00000000 # bytes(0)
                            #   ""
            "#)
        }

        hello(hex2value, value2hex) {
            DataItem::ByteString(ByteString {
                data: b"hello"[..].into(),
                bitwidth: IntegerWidth::ThirtyTwo,
            }),
            indoc!(r#"
                5a 00000005   # bytes(5)
                   68656c6c6f #   "hello"
            "#)
        }

        alpha(hex2value, value2hex) {
            DataItem::ByteString(ByteString {
                data: b"abcdefghijklmnopqrstuvwxyz"[..].into(),
                bitwidth: IntegerWidth::ThirtyTwo,
            }),
            indoc!(r#"
                5a 0000001a                         # bytes(26)
                   6162636465666768696a6b6c6d6e6f70 #   "abcdefghijklmnop"
                   7172737475767778797a             #   "qrstuvwxyz"
            "#)
        }
    }

    mod u64 {
        empty(hex2value, value2hex) {
            DataItem::ByteString(ByteString {
                data: vec![],
                bitwidth: IntegerWidth::SixtyFour,
            }),
            indoc!(r#"
                5b 0000000000000000 # bytes(0)
                                    #   ""
            "#)
        }

        hello(hex2value, value2hex) {
            DataItem::ByteString(ByteString {
                data: b"hello"[..].into(),
                bitwidth: IntegerWidth::SixtyFour,
            }),
            indoc!(r#"
                5b 0000000000000005 # bytes(5)
                   68656c6c6f       #   "hello"
            "#)
        }

        alpha(hex2value, value2hex) {
            DataItem::ByteString(ByteString {
                data: b"abcdefghijklmnopqrstuvwxyz"[..].into(),
                bitwidth: IntegerWidth::SixtyFour,
            }),
            indoc!(r#"
                5b 000000000000001a                 # bytes(26)
                   6162636465666768696a6b6c6d6e6f70 #   "abcdefghijklmnop"
                   7172737475767778797a             #   "qrstuvwxyz"
            "#)
        }
    }

    mod indefinite {
        mod diag {
            empty(value2diag) {
                DataItem::IndefiniteByteString(vec![]),
                {
                    "(_)",
                    "(_ )",
                }
            }

            one_empty(diag2value, value2diag) {
                DataItem::IndefiniteByteString(vec![
                    ByteString {
                        data: vec![],
                        bitwidth: IntegerWidth::Unknown,
                    },
                ]),
                {
                    "(_h'')",
                    "(_ h'')",
                }
            }

            some_empty(diag2value, value2diag) {
                DataItem::IndefiniteByteString(vec![
                    ByteString {
                        data: vec![],
                        bitwidth: IntegerWidth::Unknown,
                    },
                    ByteString {
                        data: vec![],
                        bitwidth: IntegerWidth::Unknown,
                    },
                ]),
                {
                    "(_h'',h'')",
                    "(_ h'', h'')",
                }
            }

            hello(diag2value, value2diag) {
                DataItem::IndefiniteByteString(vec![
                    ByteString {
                        data: b"hello"[..].into(),
                        bitwidth: IntegerWidth::Unknown,
                    },
                ]),
                {
                    "(_h'68656c6c6f')",
                    "(_ h'68656c6c6f')",
                }
            }

            hello_world(diag2value, value2diag) {
                DataItem::IndefiniteByteString(vec![
                    ByteString {
                        data: b"hello"[..].into(),
                        bitwidth: IntegerWidth::Unknown,
                    },
                    ByteString {
                        data: b"world"[..].into(),
                        bitwidth: IntegerWidth::Unknown,
                    },
                ]),
                {
                    "(_h'68656c6c6f',h'776f726c64')",
                    "(_ h'68656c6c6f', h'776f726c64')",
                }
            }

            alpha(diag2value, value2diag) {
                DataItem::IndefiniteByteString(vec![
                    ByteString {
                        data: b"abc"[..].into(),
                        bitwidth: IntegerWidth::Unknown,
                    },
                    ByteString {
                        data: b""[..].into(),
                        bitwidth: IntegerWidth::Unknown,
                    },
                    ByteString {
                        data: b"defghijklmnopqrstuv"[..].into(),
                        bitwidth: IntegerWidth::Unknown,
                    },
                    ByteString {
                        data: b"wxyz"[..].into(),
                        bitwidth: IntegerWidth::Unknown,
                    },
                ]),
                {
                    "(_h'616263',h'',h'6465666768696a6b6c6d6e6f70717273747576',h'7778797a')",
                    "
                    (_
                        h'616263',
                        h'',
                        h'6465666768696a6b6c6d6e6f70717273747576',
                        h'7778797a',
                    )
                    ",
                }
            }

            non_alpha(diag2value, value2diag) {
                DataItem::IndefiniteByteString(vec![
                    ByteString {
                        data: vec![0, 1, 2, 3, 4],
                        bitwidth: IntegerWidth::Unknown,
                    },
                    ByteString {
                        data: vec![5, 6, 7, 8, 9, 10],
                        bitwidth: IntegerWidth::Unknown,
                    },
                ]),
                {
                    "(_h'0001020304',h'05060708090a')",
                    "(_ h'0001020304', h'05060708090a')",
                }
            }
        }

        mod hex {
            empty(hex2value, value2hex) {
                DataItem::IndefiniteByteString(vec![]),
                indoc!(r#"
                    5f    # bytes(*)
                       ff #   break
                "#)
            }

            one_empty(hex2value, value2hex) {
                DataItem::IndefiniteByteString(vec![
                    ByteString {
                        data: "".into(),
                        bitwidth: IntegerWidth::Zero,
                    },
                ]),
                indoc!(r#"
                    5f    # bytes(*)
                       40 #   bytes(0)
                          #     ""
                       ff #   break
                "#)
            }

            some_empty(hex2value, value2hex) {
                DataItem::IndefiniteByteString(vec![
                    ByteString {
                        data: "".into(),
                        bitwidth: IntegerWidth::Zero,
                    },
                    ByteString {
                        data: "".into(),
                        bitwidth: IntegerWidth::Zero,
                    },
                ]),
                indoc!(r#"
                    5f    # bytes(*)
                       40 #   bytes(0)
                          #     ""
                       40 #   bytes(0)
                          #     ""
                       ff #   break
                "#)
            }

            hello_world(hex2value, value2hex) {
                DataItem::IndefiniteByteString(vec![
                    ByteString {
                        data: b"hello"[..].into(),
                        bitwidth: IntegerWidth::Zero,
                    },
                    ByteString {
                        data: b"world"[..].into(),
                        bitwidth: IntegerWidth::Sixteen,
                    },
                ]),
                indoc!(r#"
                    5f               # bytes(*)
                       45            #   bytes(5)
                          68656c6c6f #     "hello"
                       59 0005       #   bytes(5)
                          776f726c64 #     "world"
                       ff            #   break
                "#)
            }

            alpha(hex2value, value2hex) {
                DataItem::IndefiniteByteString(vec![
                    ByteString {
                        data: b"abc"[..].into(),
                        bitwidth: IntegerWidth::Zero,
                    },
                    ByteString {
                        data: "".into(),
                        bitwidth: IntegerWidth::Sixteen,
                    },
                    ByteString {
                        data: b"defghijklmnopqrstuv"[..].into(),
                        bitwidth: IntegerWidth::ThirtyTwo,
                    },
                    ByteString {
                        data: b"wxyz"[..].into(),
                        bitwidth: IntegerWidth::SixtyFour,
                    },
                ]),
                indoc!(r#"
                    5f                                     # bytes(*)
                       43                                  #   bytes(3)
                          616263                           #     "abc"
                       59 0000                             #   bytes(0)
                                                           #     ""
                       5a 00000013                         #   bytes(19)
                          6465666768696a6b6c6d6e6f70717273 #     "defghijklmnopqrs"
                          747576                           #     "tuv"
                       5b 0000000000000004                 #   bytes(4)
                          7778797a                         #     "wxyz"
                       ff                                  #   break
                "#)
            }

            non_alpha(hex2value, value2hex) {
                DataItem::IndefiniteByteString(vec![
                    ByteString {
                        data: vec![0, 1, 2, 3, 4],
                        bitwidth: IntegerWidth::Zero,
                    },
                    ByteString {
                        data: vec![5, 6, 7, 8, 9, 10],
                        bitwidth: IntegerWidth::Eight,
                    },
                ]),
                indoc!(r#"
                    5f                 # bytes(*)
                       45              #   bytes(5)
                          0001020304   #     "\x00\x01\x02\x03\x04"
                       58 06           #   bytes(6)
                          05060708090a #     "\x05\x06\x07\x08\t\n"
                       ff              #   break
                "#)
            }

            escaped(hex2value, value2hex) {
                DataItem::IndefiniteByteString(vec![
                    ByteString {
                        data: b"\\"[..].into(),
                        bitwidth: IntegerWidth::Zero,
                    },
                    ByteString {
                        data: b"\""[..].into(),
                        bitwidth: IntegerWidth::Eight,
                    },
                ]),
                indoc!(r#"
                    5f       # bytes(*)
                       41    #   bytes(1)
                          5c #     "\\"
                       58 01 #   bytes(1)
                          22 #     "\""
                       ff    #   break
                "#)
            }
        }
    }

    mod encodings {
        base16(diag2value) {
            DataItem::ByteString(ByteString {
                data: hex!("12345678").into(),
                bitwidth: IntegerWidth::Unknown,
            }),
            { "h'12345678'" }
        }

        base32(diag2value) {
            DataItem::ByteString(ByteString {
                data: hex!("12345678").into(),
                bitwidth: IntegerWidth::Unknown,
            }),
            { "b32'CI2FM6A='" }
        }

        base32hex(diag2value) {
            DataItem::ByteString(ByteString {
                data: hex!("12345678").into(),
                bitwidth: IntegerWidth::Unknown,
            }),
            { "h32'28Q5CU0='" }
        }

        base64(diag2value) {
            DataItem::ByteString(ByteString {
                data: hex!("12345678").into(),
                bitwidth: IntegerWidth::Unknown,
            }),
            { "b64'EjRWeA=='" }
        }

        base64url(diag2value) {
            DataItem::ByteString(ByteString {
                data: hex!("12345678").into(),
                bitwidth: IntegerWidth::Unknown,
            }),
            { "b64'EjRWeA'" }
        }

        // RFC 8610 Appendix G.1
        mod whitespace {
            base16(diag2value) {
                DataItem::ByteString(ByteString {
                    data: hex!("12345678").into(),
                    bitwidth: IntegerWidth::Unknown,
                }),
                { "h'12 34\t56\n78'" }
            }

            base32(diag2value) {
                DataItem::ByteString(ByteString {
                    data: hex!("12345678").into(),
                    bitwidth: IntegerWidth::Unknown,
                }),
                { "b32'CI 2F\tM6A\n='" }
            }

            base32hex(diag2value) {
                DataItem::ByteString(ByteString {
                    data: hex!("12345678").into(),
                    bitwidth: IntegerWidth::Unknown,
                }),
                { "h32'28 Q5\tCU0\n='" }
            }

            base64(diag2value) {
                DataItem::ByteString(ByteString {
                    data: hex!("12345678").into(),
                    bitwidth: IntegerWidth::Unknown,
                }),
                { "b64'Ej RW\teA\n=='" }
            }

            base64url(diag2value) {
                DataItem::ByteString(ByteString {
                    data: hex!("12345678").into(),
                    bitwidth: IntegerWidth::Unknown,
                }),
                { "b64'Ej RW\teA\n'" }
            }
        }
    }
}
