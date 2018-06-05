use std::str::FromStr;

use nom;

use {Error, Result, Simple, Value};

#[inline]
pub fn decimal<T: FromStr>(input: &str) -> nom::IResult<&str, T> {
    let (remaining, parsed) = is_a!(input, "0123456789")?;
    let value = parsed
        .parse()
        .map_err(|_| nom::Err::Failure(nom::Context::Code(input, nom::ErrorKind::Custom(0))))?;
    Ok((remaining, value))
}

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
        value: delimited!(tag!("("), call!(decimal::<u8>), tag!(")")) >>
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
