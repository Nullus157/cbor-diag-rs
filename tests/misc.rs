#[macro_use]
extern crate indoc;
#[macro_use]
extern crate pretty_assertions;

extern crate cbor_diag;

use cbor_diag::{DataItem, Simple};

#[macro_use]
mod utils;

testcases! {
    false_ {
        DataItem::Simple(Simple::FALSE),
        {
            "false",
            "false",
        },
        indoc!("
            f4 # false, simple(20)
        "),
    }

    true_ {
        DataItem::Simple(Simple::TRUE),
        {
            "true",
            "true",
        },
        indoc!("
            f5 # true, simple(21)
        "),
    }

    null {
        DataItem::Simple(Simple::NULL),
        {
            "null",
            "null",
        },
        indoc!("
            f6 # null, simple(22)
        "),
    }

    undefined {
        DataItem::Simple(Simple::UNDEFINED),
        {
            "undefined",
            "undefined",
        },
        indoc!("
            f7 # undefined, simple(23)
        "),
    }

    simple_16 {
        DataItem::Simple(Simple(16)),
        {
            "simple(16)",
            "simple(16)",
        },
        indoc!("
            f0 # unassigned, simple(16)
        "),
    }

    simple_24 {
        DataItem::Simple(Simple(24)),
        {
            "simple(24)",
            "simple(24)",
        },
        indoc!("
            f8 18 # reserved, simple(24)
        "),
    }

    simple_255 {
        DataItem::Simple(Simple(255)),
        {
            "simple(255)",
            "simple(255)",
        },
        indoc!("
            f8 ff # unassigned, simple(255)
        "),
    }
}
