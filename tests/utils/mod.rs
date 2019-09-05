extern crate cbor_diag;

pub use cbor_diag::{parse_diag, parse_hex};

#[derive(Eq)]
#[doc(hidden)]
pub struct DisplayDebug<T>(pub T);

impl<T, U> PartialEq<DisplayDebug<U>> for DisplayDebug<T>
where
    T: PartialEq<U>,
{
    fn eq(&self, rhs: &DisplayDebug<U>) -> bool {
        self.0.eq(&rhs.0)
    }
}

impl<T: std::fmt::Display> std::fmt::Debug for DisplayDebug<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[macro_export]
macro_rules! testcases {
    (
        @testcase $name:ident(diag2value $(, $rest:ident)+) {
            $value:expr, $diag:expr, $hex:expr
        }
    ) => {
        #[test]
        fn diag2value() {
            let value = $crate::utils::parse_diag($diag).unwrap();
            assert_eq!(value, $value);
        }

        testcases! {
            @testcase $name($($rest),*) { $value, $diag, $hex }
        }
    };

    (
        @testcase $name:ident(hex2value $(, $rest:ident)+) {
            $value:expr, $diag:expr, $hex:expr
        }
    ) => {
        #[test]
        fn hex2value() {
            let value = $crate::utils::parse_hex($hex).unwrap();
            assert_eq!(value, $value);
        }

        testcases! {
            @testcase $name($($rest),*) { $value, $diag, $hex }
        }
    };

    (
        @testcase $name:ident(value2diag $(, $rest:ident)*) {
            $value:expr, $diag:expr, $hex:expr
        }
    ) => {
        #[test]
        fn value2diag() {
            let diag = $value.to_diag();
            assert_eq!($crate::utils::DisplayDebug(diag), $crate::utils::DisplayDebug($diag));
        }

        testcases! {
            @testcase $name($($rest),*) { $value, $diag, $hex }
        }
    };

    (
        @testcase $name:ident(value2hex $(, $rest:ident)*) {
            $value:expr, $diag:expr, $hex:expr
        }
    ) => {
        #[test]
        fn value2hex() {
            let hex = $value.to_hex();
            assert_eq!($crate::utils::DisplayDebug(hex), $crate::utils::DisplayDebug($hex));
        }

        testcases! {
            @testcase $name($($rest),*) { $value, $diag, $hex }
        }
    };

    (
        @testcase $name:ident() {
            $value:expr, $diag:expr, $hex:expr
        }
    ) => {
    };

    (
        @testcases $name:ident($($test:ident),+) {
            $value:expr, $diag:expr, $hex:expr $(,)*
        }
    ) => {
        mod $name {
            #[allow(unused_imports)]
            use super::*;

            testcases! {
                @testcase $name($($test),+) { $value, $diag, $hex }
            }
        }
    };

    (
        @testcases $name:ident($($test:ident),+) {
            $value:expr, $hexordiag:expr $(,)*
        }
    ) => {
        mod $name {
            #[allow(unused_imports)]
            use super::*;

            testcases! {
                @testcase $name($($test),+) { $value, $hexordiag, $hexordiag }
            }
        }
    };

    ($name:ident($($test:ident),+) { $($tt:tt)+ } $($rest:tt)*) => {
        testcases! {
            @testcases $name($($test),+) { $($tt)+ }
        }
        testcases! { $($rest)* }
    };

    ($name:ident { $($tt:tt)+ } $($rest:tt)*) => {
        testcases! {
            @testcases $name(diag2value, hex2value, value2diag, value2hex) {
                $($tt)+
            }
        }
        testcases! { $($rest)* }
    };

    ($(mod $name:ident { $($tt:tt)* })*) => {
        $(
        mod $name {
            #[allow(unused_imports)]
            use super::*;

            testcases! { $($tt)* }
        }
        )*
    };
}
