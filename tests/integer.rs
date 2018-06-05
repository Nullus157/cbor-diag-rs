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
            "00 # unsigned(0)",
        }

        one {
            Value::Integer {
                value: 1,
                bitwidth: IntegerWidth::Zero,
            },
            "1",
            "01 # unsigned(1)",
        }

        twenty_three {
            Value::Integer {
                value: 23,
                bitwidth: IntegerWidth::Zero,
            },
            "23",
            "17 # unsigned(23)",
        }
    }

    mod u8 {
        zero {
            Value::Integer {
                value: 0,
                bitwidth: IntegerWidth::Eight,
            },
            "0_0",
            "18 00 # unsigned(0)",
        }

        one {
            Value::Integer {
                value: 1,
                bitwidth: IntegerWidth::Eight,
            },
            "1_0",
            "18 01 # unsigned(1)",
        }

        twenty_four {
            Value::Integer {
                value: 24,
                bitwidth: IntegerWidth::Eight,
            },
            "24_0",
            "18 18 # unsigned(24)",
        }
    }

    mod u16 {
        zero {
            Value::Integer {
                value: 0,
                bitwidth: IntegerWidth::Sixteen,
            },
            "0_1",
            "19 00 00 # unsigned(0)",
        }

        one {
            Value::Integer {
                value: 1,
                bitwidth: IntegerWidth::Sixteen,
            },
            "1_1",
            "19 00 01 # unsigned(1)",
        }

        twenty_four {
            Value::Integer {
                value: 24,
                bitwidth: IntegerWidth::Sixteen,
            },
            "24_1",
            "19 00 18 # unsigned(24)",
        }
    }

    mod u32 {
        zero {
            Value::Integer {
                value: 0,
                bitwidth: IntegerWidth::ThirtyTwo,
            },
            "0_2",
            "1a 00 00 00 00 # unsigned(0)",
        }

        one {
            Value::Integer {
                value: 1,
                bitwidth: IntegerWidth::ThirtyTwo,
            },
            "1_2",
            "1a 00 00 00 01 # unsigned(1)",
        }

        twenty_four {
            Value::Integer {
                value: 24,
                bitwidth: IntegerWidth::ThirtyTwo,
            },
            "24_2",
            "1a 00 00 00 18 # unsigned(24)",
        }
    }

    mod u64 {
        zero {
            Value::Integer {
                value: 0,
                bitwidth: IntegerWidth::SixtyFour,
            },
            "0_3",
            "1b 00 00 00 00 00 00 00 00 # unsigned(0)",
        }

        one {
            Value::Integer {
                value: 1,
                bitwidth: IntegerWidth::SixtyFour,
            },
            "1_3",
            "1b 00 00 00 00 00 00 00 01 # unsigned(1)",
        }

        twenty_four {
            Value::Integer {
                value: 24,
                bitwidth: IntegerWidth::SixtyFour,
            },
            "24_3",
            "1b 00 00 00 00 00 00 00 18 # unsigned(24)",
        }
    }
}
