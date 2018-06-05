use std::str::FromStr;

use nom::{self, digit};

use {Error, IntegerWidth, Result, Simple, Value};

type NStr<'a> = nom::types::CompleteStr<'a>;

fn parse<T: FromStr>(s: NStr) -> ::std::result::Result<T, T::Err> {
    T::from_str(s.0)
}

named! {
    integer<NStr, Value>,
    alt_complete!(
        do_parse!(
            value: map_res!(digit, parse::<u64>) >>
            tag!("_") >>
            encoding: verify!(map_res!(digit, parse::<u64>), |e| e < 4) >>
            (Value::Integer {
                value,
                bitwidth: match encoding {
                    0 => IntegerWidth::Eight,
                    1 => IntegerWidth::Sixteen,
                    2 => IntegerWidth::ThirtyTwo,
                    3 => IntegerWidth::SixtyFour,
                    _ => unreachable!(),
                }
            })
        )
        | map!(
            map_res!(digit, parse::<u64>),
            |value| Value::Integer {
                value,
                bitwidth: match value {
                    0...23 => IntegerWidth::Zero,
                    _ => IntegerWidth::Unknown,
                }
            })
    )
}

named! {
    simple<NStr, Value>,
    map!(
        alt_complete!(
            value!(Simple::FALSE, tag!("false"))
            | value!(Simple::TRUE, tag!("true"))
            | value!(Simple::NULL, tag!("null"))
            | value!(Simple::UNDEFINED, tag!("undefined"))
            | map!(preceded!(tag!("simple"),
                map_res!(delimited!(tag!("("), digit, tag!(")")), parse::<u8>)),
                Simple)
        ),
        Value::Simple)
}

named! {
    value<NStr, Value>,
    alt_complete!(integer | simple)
}

pub fn parse_diag(text: impl AsRef<str>) -> Result<Value> {
    let text = nom::types::CompleteStr(text.as_ref());
    let (remaining, parsed) = value(text).map_err(|e| {
        println!("{}: {:?}", e, e);
        Error::Todos("Parsing error")
    })?;
    if !remaining.is_empty() {
        return Err(Error::Todos("Remaining text"));
    }
    Ok(parsed)
}
