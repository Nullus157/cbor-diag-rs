#[macro_use]
extern crate indoc;
#[macro_use]
extern crate pretty_assertions;

extern crate cbor_diag;

use cbor_diag::{IntegerWidth, Value};

#[macro_use]
mod utils;

testcases! {
    mod utiny {
        zero {
            Value::Integer {
                value: 0,
                bitwidth: IntegerWidth::Zero,
            },
            "0",
            indoc!("
                00 # unsigned(0)
            ")
        }

        one {
            Value::Integer {
                value: 1,
                bitwidth: IntegerWidth::Zero,
            },
            "1",
            indoc!("
                01 # unsigned(1)
            ")
        }

        twenty_three {
            Value::Integer {
                value: 23,
                bitwidth: IntegerWidth::Zero,
            },
            "23",
            indoc!("
                17 # unsigned(23)
            ")
        }
    }

    mod u8 {
        zero {
            Value::Integer {
                value: 0,
                bitwidth: IntegerWidth::Eight,
            },
            "0_0",
            indoc!("
                18 00 # unsigned(0)
            ")
        }

        one {
            Value::Integer {
                value: 1,
                bitwidth: IntegerWidth::Eight,
            },
            "1_0",
            indoc!("
                18 01 # unsigned(1)
            ")
        }

        twenty_four {
            Value::Integer {
                value: 24,
                bitwidth: IntegerWidth::Eight,
            },
            "24_0",
            indoc!("
                18 18 # unsigned(24)
            ")
        }
    }

    mod u16 {
        zero {
            Value::Integer {
                value: 0,
                bitwidth: IntegerWidth::Sixteen,
            },
            "0_1",
            indoc!("
                19 0000 # unsigned(0)
            ")
        }

        one {
            Value::Integer {
                value: 1,
                bitwidth: IntegerWidth::Sixteen,
            },
            "1_1",
            indoc!("
                19 0001 # unsigned(1)
            ")
        }

        twenty_four {
            Value::Integer {
                value: 24,
                bitwidth: IntegerWidth::Sixteen,
            },
            "24_1",
            indoc!("
                19 0018 # unsigned(24)
            ")
        }
    }

    mod u32 {
        zero {
            Value::Integer {
                value: 0,
                bitwidth: IntegerWidth::ThirtyTwo,
            },
            "0_2",
            indoc!("
                1a 00000000 # unsigned(0)
            ")
        }

        one {
            Value::Integer {
                value: 1,
                bitwidth: IntegerWidth::ThirtyTwo,
            },
            "1_2",
            indoc!("
                1a 00000001 # unsigned(1)
            ")
        }

        twenty_four {
            Value::Integer {
                value: 24,
                bitwidth: IntegerWidth::ThirtyTwo,
            },
            "24_2",
            indoc!("
                1a 00000018 # unsigned(24)
            ")
        }
    }

    mod u64 {
        zero {
            Value::Integer {
                value: 0,
                bitwidth: IntegerWidth::SixtyFour,
            },
            "0_3",
            indoc!("
                1b 0000000000000000 # unsigned(0)
            ")
        }

        one {
            Value::Integer {
                value: 1,
                bitwidth: IntegerWidth::SixtyFour,
            },
            "1_3",
            indoc!("
                1b 0000000000000001 # unsigned(1)
            ")
        }

        twenty_four {
            Value::Integer {
                value: 24,
                bitwidth: IntegerWidth::SixtyFour,
            },
            "24_3",
            indoc!("
                1b 0000000000000018 # unsigned(24)
            ")
        }
    }

    mod negative_utiny {
        one {
            Value::Negative {
                value: 0,
                bitwidth: IntegerWidth::Zero,
            },
            "-1",
            indoc!("
                20 # negative(-1)
            ")
        }

        twenty_four {
            Value::Negative {
                value: 23,
                bitwidth: IntegerWidth::Zero,
            },
            "-24",
            indoc!("
                37 # negative(-24)
            ")
        }
    }

    mod negative_u8 {
        one {
            Value::Negative {
                value: 0,
                bitwidth: IntegerWidth::Eight,
            },
            "-1_0",
            indoc!("
                38 00 # negative(-1)
            ")
        }

        twenty_five {
            Value::Negative {
                value: 24,
                bitwidth: IntegerWidth::Eight,
            },
            "-25_0",
            indoc!("
                38 18 # negative(-25)
            ")
        }
    }

    mod negative_u16 {
        one {
            Value::Negative {
                value: 0,
                bitwidth: IntegerWidth::Sixteen,
            },
            "-1_1",
            indoc!("
                39 0000 # negative(-1)
            ")
        }

        twenty_five {
            Value::Negative {
                value: 24,
                bitwidth: IntegerWidth::Sixteen,
            },
            "-25_1",
            indoc!("
                39 0018 # negative(-25)
            ")
        }
    }

    mod negative_u32 {
        one {
            Value::Negative {
                value: 0,
                bitwidth: IntegerWidth::ThirtyTwo,
            },
            "-1_2",
            indoc!("
                3a 00000000 # negative(-1)
            ")
        }

        twenty_five {
            Value::Negative {
                value: 24,
                bitwidth: IntegerWidth::ThirtyTwo,
            },
            "-25_2",
            indoc!("
                3a 00000018 # negative(-25)
            ")
        }
    }

    mod negative_u64 {
        one {
            Value::Negative {
                value: 0,
                bitwidth: IntegerWidth::SixtyFour,
            },
            "-1_3",
            indoc!("
                3b 0000000000000000 # negative(-1)
            ")
        }

        twenty_five {
            Value::Negative {
                value: 24,
                bitwidth: IntegerWidth::SixtyFour,
            },
            "-25_3",
            indoc!("
                3b 0000000000000018 # negative(-25)
            ")
        }
    }
}
