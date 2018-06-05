use std::str::FromStr;

use nom::digit;

use {Error, Result, Simple, Value};

named! {
    false_<&str, Simple>,
    do_parse!(tag!("false") >> (Simple::FALSE))
}

named! {
    true_<&str, Simple>,
    do_parse!(tag!("true") >> (Simple::TRUE))
}

named! {
    null<&str, Simple>,
    do_parse!(tag!("null") >> (Simple::NULL))
}

named! {
    undefined<&str, Simple>,
    do_parse!(tag!("undefined") >> (Simple::UNDEFINED))
}

named! {
    simple<&str, Simple>,
    do_parse!(
        tag!("simple") >>
        value: map_res!(delimited!(tag!("("), digit, tag!(")")), u8::from_str) >>
        (Simple(value)))
}

named! {
    simple_value<&str, Value>,
    do_parse!(
        simple: alt!(false_ | true_ | null | undefined | simple) >>
        (Value::Simple(simple)))
}

pub fn parse_diag(text: impl AsRef<str>) -> Result<Value> {
    let (remaining, parsed) = simple_value(text.as_ref()).map_err(|e| {
        println!("{}: {:?}", e, e);
        Error::Todos("Parsing error")
    })?;
    if !remaining.is_empty() {
        return Err(Error::Todos("Remaining text"));
    }
    Ok(parsed)
}
