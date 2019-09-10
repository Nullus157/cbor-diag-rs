extern crate cbor_diag;
pub extern crate hex;

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

pub fn remove_comments(hex: impl AsRef<str>) -> String {
    hex.as_ref()
        .lines()
        .map(|line| line.split('#').next().unwrap().replace(" ", ""))
        .collect()
}

#[macro_export]
macro_rules! testcases {
    (
        @testcase $name:ident(diag2value $(, $rest:ident)+) {
            $value:expr, { $compact:expr $(, $pretty:expr)? }, { $hex:expr }
        }
    ) => {
        #[test]
        fn diag2value_compact() {
            let value = $crate::utils::parse_diag($compact).unwrap();
            assert_eq!(value, $value);
        }

        $(
            #[test]
            fn diag2value_pretty() {
                let value = $crate::utils::parse_diag($pretty).unwrap();
                assert_eq!(value, $value);
            }
        )?

        testcases! {
            @testcase $name($($rest),*) { $value, { $compact $(, $pretty)? }, { $hex } }
        }
    };

    (
        @testcase $name:ident(hex2value $(, $rest:ident)+) {
            $value:expr, { $compact:expr $(, $pretty:expr)? }, { $hex:expr }
        }
    ) => {
        #[test]
        fn hex2value() {
            let value = $crate::utils::parse_hex($hex).unwrap();
            assert_eq!(value, $value);
        }

        testcases! {
            @testcase $name($($rest),*) { $value, { $compact $(, $pretty)? }, { $hex } }
        }
    };

    (
        @testcase $name:ident(value2diag $(, $rest:ident)*) {
            $value:expr, { $compact:expr $(, $pretty:expr)? }, { $hex:expr }
        }
    ) => {
        #[test]
        fn value2diag_compact() {
            let compact = $value.to_diag();
            assert_eq!($crate::utils::DisplayDebug(compact), $crate::utils::DisplayDebug($compact));
        }

        $(
            #[test]
            fn value2diag_pretty() {
                let pretty = $value.to_diag_pretty();
                assert_eq!($crate::utils::DisplayDebug(pretty), $crate::utils::DisplayDebug(indoc!($pretty).trim()));
            }
        )?

        testcases! {
            @testcase $name($($rest),*) { $value, { $compact $(, $pretty)? }, { $hex } }
        }
    };

    (
        @testcase $name:ident(value2diag $(, $rest:ident)*) {
            $value:expr, { $compact:expr $(, $pretty:expr)? }, { $hex:expr }
        }
    ) => {
        #[test]
        fn value2diag() {
            let diag = $value.to_diag();
            assert_eq!($crate::utils::DisplayDebug(diag), $crate::utils::DisplayDebug($diag));
        }

        testcases! {
            @testcase $name($($rest),*) { $value, { $compact $(, $pretty)? }, { $hex } }
        }
    };

    (
        @testcase $name:ident(value2hex $(, $rest:ident)*) {
            $value:expr, { $compact:expr $(, $pretty:expr)? }, { $hex:expr }
        }
    ) => {
        #[test]
        fn value2bytes() {
            let hex = $crate::utils::hex::encode($value.to_bytes());
            let expected = $crate::utils::remove_comments($hex);
            assert_eq!($crate::utils::DisplayDebug(hex), $crate::utils::DisplayDebug(expected));
        }

        #[test]
        fn value2hex() {
            let hex = $value.to_hex();
            assert_eq!($crate::utils::DisplayDebug(hex), $crate::utils::DisplayDebug($hex));
        }

        testcases! {
            @testcase $name($($rest),*) { $value, { $compact $(, $pretty)? }, { $hex } }
        }
    };

    (
        @testcase $name:ident() {
            $value:expr, { $compact:expr $(, $pretty:expr)? }, { $hex:expr }
        }
    ) => {
    };

    (
        @testcases $name:ident($($test:ident),+) {
            $value:expr, { $compact:expr $(, $pretty:expr)? }, { $hex:expr }
        }
    ) => {
        mod $name {
            #[allow(unused_imports)]
            use super::*;

            testcases! {
                @testcase $name($($test),*) { $value, { $compact $(, $pretty)? }, { $hex } }
            }
        }
    };

    (
        @testcases $name:ident($($test:ident),+) {
            $value:expr, { $compact:expr, $pretty:expr $(,)+ }, $hex:expr $(,)*
        }
    ) => {
        mod $name {
            #[allow(unused_imports)]
            use super::*;

            testcases! {
                @testcase $name($($test),*) { $value, { $compact, $pretty }, { $hex } }
            }
        }
    };

    (
        @testcases $name:ident($($test:ident),+) {
            $value:expr, { $compact:expr, $pretty:expr $(,)* } $(,)*
        }
    ) => {
        mod $name {
            #[allow(unused_imports)]
            use super::*;

            testcases! {
                @testcase $name($($test),+) { $value, { $compact, $pretty }, { "" } }
            }
        }
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
                @testcase $name($($test),+) { $value, { $diag }, { $hex } }
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
                @testcase $name($($test),+) { $value, { $hexordiag }, { $hexordiag } }
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
