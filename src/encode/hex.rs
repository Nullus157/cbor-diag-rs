use std::{
    ascii, cmp,
    convert::TryFrom,
    i64, iter,
    net::{Ipv4Addr, Ipv6Addr},
};

use super::Encoding;
use chrono::{DateTime, NaiveDate, NaiveDateTime};
use half::f16;
use num_bigint::{BigInt, BigUint, Sign};
use num_rational::{BigRational, Ratio};
use num_traits::pow::pow;
use separator::Separatable;
use url::Url;
use uuid::Uuid;

use crate::{parse_bytes, ByteString, DataItem, FloatWidth, IntegerWidth, Simple, Tag, TextString};

struct Context {
    encoding: Option<Encoding>,
    reference_count: u64,
}

struct Line {
    hex: String,
    comment: String,
    sublines: Vec<Line>,
}

impl Line {
    fn new(hex: impl Into<String>, comment: impl Into<String>) -> Line {
        Line {
            hex: hex.into(),
            comment: comment.into(),
            sublines: Vec::new(),
        }
    }

    fn from_value(context: &mut Context, value: &DataItem) -> Line {
        match *value {
            DataItem::Integer { value, bitwidth } => integer_to_hex(value, bitwidth),
            DataItem::Negative { value, bitwidth } => negative_to_hex(value, bitwidth),
            DataItem::ByteString(ref bytestring) => {
                definite_bytestring_to_hex(context.encoding, bytestring)
            }
            DataItem::IndefiniteByteString(ref bytestrings) => {
                indefinite_string_to_hex(0x02, "bytes", bytestrings, |bytestring| {
                    definite_bytestring_to_hex(context.encoding, bytestring)
                })
            }
            DataItem::TextString(ref textstring) => definite_textstring_to_hex(textstring),
            DataItem::IndefiniteTextString(ref textstrings) => {
                indefinite_string_to_hex(0x03, "text", textstrings, definite_textstring_to_hex)
            }
            DataItem::Array { ref data, bitwidth } => array_to_hex(context, data, bitwidth),
            DataItem::Map { ref data, bitwidth } => map_to_hex(context, data, bitwidth),
            DataItem::Tag {
                tag,
                bitwidth,
                ref value,
            } => tagged_to_hex(context, tag, bitwidth, value),
            DataItem::Float { value, bitwidth } => float_to_hex(value, bitwidth),
            DataItem::Simple(simple) => simple_to_hex(simple),
        }
    }

    fn merge(self) -> String {
        let hex_width = self.hex_width();
        let mut output = String::with_capacity(128);
        self.do_merge(hex_width as isize, 0, &mut output);
        output
    }

    fn do_merge(self, hex_width: isize, indent_level: usize, output: &mut String) {
        use std::fmt::Write;

        let (hex_indent, width) = if hex_width < 0 {
            (indent_level * 3 - hex_width.unsigned_abs(), 0)
        } else {
            (indent_level * 3, hex_width as usize)
        };

        writeln!(
            output,
            "{blank:hex_indent$}{hex:width$} # {blank:comment_indent$}{comment}",
            blank = "",
            hex_indent = hex_indent,
            comment_indent = indent_level * 2,
            hex = self.hex,
            width = width,
            comment = self.comment
        )
        .unwrap();

        for line in self.sublines {
            line.do_merge(hex_width - 3, indent_level + 1, output);
        }
    }

    fn hex_width(&self) -> usize {
        cmp::max(
            self.hex.len(),
            self.sublines
                .iter()
                .map(|line| {
                    let subwidth = line.hex_width();
                    if subwidth == 0 {
                        0
                    } else {
                        subwidth + 3
                    }
                })
                .max()
                .unwrap_or(0),
        )
    }
}

fn integer_to_hex(value: u64, mut bitwidth: IntegerWidth) -> Line {
    if bitwidth == IntegerWidth::Unknown {
        bitwidth = if value < 24 {
            IntegerWidth::Zero
        } else if value < u64::from(u8::max_value()) {
            IntegerWidth::Eight
        } else if value < u64::from(u16::max_value()) {
            IntegerWidth::Sixteen
        } else if value < u64::from(u32::max_value()) {
            IntegerWidth::ThirtyTwo
        } else {
            IntegerWidth::SixtyFour
        };
    }

    let hex = match bitwidth {
        IntegerWidth::Unknown => unreachable!(),
        IntegerWidth::Zero => format!("{:02x}", value),
        IntegerWidth::Eight => format!("18 {:02x}", value),
        IntegerWidth::Sixteen => format!("19 {:04x}", value),
        IntegerWidth::ThirtyTwo => format!("1a {:08x}", value),
        IntegerWidth::SixtyFour => format!("1b {:016x}", value),
    };

    let comment = format!("unsigned({})", value.separated_string());

    Line::new(hex, comment)
}

fn negative_to_hex(value: u64, mut bitwidth: IntegerWidth) -> Line {
    if bitwidth == IntegerWidth::Unknown {
        bitwidth = if value < 24 {
            IntegerWidth::Zero
        } else if value < u64::from(u8::max_value()) {
            IntegerWidth::Eight
        } else if value < u64::from(u16::max_value()) {
            IntegerWidth::Sixteen
        } else if value < u64::from(u32::max_value()) {
            IntegerWidth::ThirtyTwo
        } else {
            IntegerWidth::SixtyFour
        };
    }

    let hex = match bitwidth {
        IntegerWidth::Unknown => unreachable!(),
        IntegerWidth::Zero => format!("{:02x}", value + 0x20),
        IntegerWidth::Eight => format!("38 {:02x}", value),
        IntegerWidth::Sixteen => format!("39 {:04x}", value),
        IntegerWidth::ThirtyTwo => format!("3a {:08x}", value),
        IntegerWidth::SixtyFour => format!("3b {:016x}", value),
    };

    let comment = format!("negative({})", (-1 - i128::from(value)).separated_string());

    Line::new(hex, comment)
}

fn length_to_hex(
    length: Option<usize>,
    mut bitwidth: Option<IntegerWidth>,
    major: u8,
    kind: &str,
) -> Line {
    // TODO: Rearrange the data to remove the unwraps.

    if bitwidth == Some(IntegerWidth::Unknown) {
        bitwidth = if length.unwrap() < 24 {
            Some(IntegerWidth::Zero)
        } else if length.unwrap() < usize::from(u8::max_value()) {
            Some(IntegerWidth::Eight)
        } else if length.unwrap() < usize::from(u16::max_value()) {
            Some(IntegerWidth::Sixteen)
        } else if length.unwrap() < u32::max_value() as usize {
            Some(IntegerWidth::ThirtyTwo)
        } else {
            Some(IntegerWidth::SixtyFour)
        };
    }

    let hex = match bitwidth {
        Some(IntegerWidth::Unknown) => unreachable!(),
        Some(IntegerWidth::Zero) => format!("{:02x}", (length.unwrap() as u8) + (major << 5)),
        Some(IntegerWidth::Eight) => format!("{:02x} {:02x}", (major << 5) | 0x18, length.unwrap()),
        Some(IntegerWidth::Sixteen) => {
            format!("{:02x} {:04x}", (major << 5) | 0x19, length.unwrap())
        }
        Some(IntegerWidth::ThirtyTwo) => {
            format!("{:02x} {:08x}", (major << 5) | 0x1a, length.unwrap())
        }
        Some(IntegerWidth::SixtyFour) => {
            format!("{:02x} {:016x}", (major << 5) | 0x1b, length.unwrap())
        }
        None => format!("{:02x}", (major << 5) | 0x1F),
    };

    let comment = format!(
        "{kind}({length})",
        kind = kind,
        length = if bitwidth.is_some() {
            length.unwrap().to_string()
        } else {
            "*".to_owned()
        },
    );

    Line::new(hex, comment)
}

fn bytes_to_hex(encoding: Option<Encoding>, data: &[u8]) -> impl Iterator<Item = Line> + '_ {
    data.chunks(16).map(move |datum| {
        let hex = data_encoding::HEXLOWER.encode(datum);
        let comment = match encoding {
            Some(Encoding::Base64Url) => {
                let mut comment = "b64'".to_owned();
                data_encoding::BASE64URL_NOPAD.encode_append(data, &mut comment);
                comment.push('\'');
                comment
            }
            Some(Encoding::Base64) => {
                let mut comment = "b64'".to_owned();
                data_encoding::BASE64.encode_append(data, &mut comment);
                comment.push('\'');
                comment
            }
            Some(Encoding::Base16) => format!("h'{}'", hex),
            None => {
                let text: String = datum
                    .iter()
                    .cloned()
                    .flat_map(ascii::escape_default)
                    .map(char::from)
                    .collect();
                format!("\"{}\"", text)
            }
        };
        Line::new(hex, comment)
    })
}

fn definite_bytestring_to_hex(encoding: Option<Encoding>, bytestring: &ByteString) -> Line {
    let ByteString { ref data, bitwidth } = *bytestring;

    let mut line = length_to_hex(Some(data.len()), Some(bitwidth), 2, "bytes");

    if data.is_empty() {
        line.sublines.push(Line::new("", "\"\""));
    } else {
        line.sublines.extend(bytes_to_hex(encoding, data))
    }

    line
}

fn definite_textstring_to_hex(textstring: &TextString) -> Line {
    let TextString { ref data, bitwidth } = *textstring;

    let mut line = length_to_hex(Some(data.len()), Some(bitwidth), 3, "text");

    if data.is_empty() {
        line.sublines.push(Line::new("", "\"\""));
    } else {
        let mut push_line = |datum: &str| {
            let hex = data_encoding::HEXLOWER.encode(datum.as_bytes());
            let mut comment = String::with_capacity(datum.len());
            comment.push('"');
            for c in datum.chars() {
                if c == '\"' || c == '\\' || c.is_control() {
                    for c in c.escape_default() {
                        comment.push(c);
                    }
                } else {
                    comment.push(c);
                }
            }
            comment.push('"');
            line.sublines.push(Line::new(hex, comment));
        };

        if data.len() <= 24 {
            push_line(data);
        } else {
            let mut data = data.as_str();
            while !data.is_empty() {
                let mut split = 16;
                while !data.is_char_boundary(split) {
                    split -= 1;
                }
                let (datum, new_data) = data.split_at(split);
                data = new_data;
                push_line(datum);
            }
        }
    }

    line
}

fn indefinite_string_to_hex<T>(
    major: u8,
    name: &str,
    strings: &[T],
    definite_string_to_hex: impl Fn(&T) -> Line,
) -> Line {
    let mut line = length_to_hex(None, None, major, name);

    line.sublines
        .extend(strings.iter().map(definite_string_to_hex));
    line.sublines.push(Line::new("ff", "break"));

    line
}

fn array_to_hex(context: &mut Context, array: &[DataItem], bitwidth: Option<IntegerWidth>) -> Line {
    let mut line = length_to_hex(Some(array.len()), bitwidth, 4, "array");

    line.sublines
        .extend(array.iter().map(|value| Line::from_value(context, value)));

    if bitwidth.is_none() {
        line.sublines.push(Line::new("ff", "break"));
    }

    line
}

fn map_to_hex(
    context: &mut Context,
    values: &[(DataItem, DataItem)],
    bitwidth: Option<IntegerWidth>,
) -> Line {
    let mut line = length_to_hex(Some(values.len()), bitwidth, 5, "map");

    line.sublines.extend(
        values
            .iter()
            .flat_map(|(v1, v2)| iter::once(v1).chain(iter::once(v2)))
            .map(|value| Line::from_value(context, value)),
    );

    if bitwidth.is_none() {
        line.sublines.push(Line::new("ff", "break"));
    }

    line
}

fn tagged_to_hex(
    context: &mut Context,
    tag: Tag,
    mut bitwidth: IntegerWidth,
    value: &DataItem,
) -> Line {
    if bitwidth == IntegerWidth::Unknown {
        bitwidth = if tag.0 < 24 {
            IntegerWidth::Zero
        } else if tag.0 < u64::from(u8::max_value()) {
            IntegerWidth::Eight
        } else if tag.0 < u64::from(u16::max_value()) {
            IntegerWidth::Sixteen
        } else if tag.0 < u64::from(u32::max_value()) {
            IntegerWidth::ThirtyTwo
        } else {
            IntegerWidth::SixtyFour
        };
    }

    let hex = match bitwidth {
        IntegerWidth::Unknown => unreachable!(),
        IntegerWidth::Zero => format!("{:02x}", 0xc0 | tag.0),
        IntegerWidth::Eight => format!("d8 {:02x}", tag.0),
        IntegerWidth::Sixteen => format!("d9 {:04x}", tag.0),
        IntegerWidth::ThirtyTwo => format!("da {:08x}", tag.0),
        IntegerWidth::SixtyFour => format!("db {:016x}", tag.0),
    };

    let extra = match tag {
        Tag::DATETIME => Some("standard datetime string"),
        Tag::EPOCH_DATETIME => Some("epoch datetime value"),
        Tag::POSITIVE_BIGNUM => Some("positive bignum"),
        Tag::NEGATIVE_BIGNUM => Some("negative bignum"),
        Tag::DECIMAL_FRACTION => Some("decimal fraction"),
        Tag::BIGFLOAT => Some("bigfloat"),
        Tag::ENCODED_BASE64URL => Some("suggested base64url encoding"),
        Tag::ENCODED_BASE64 => Some("suggested base64 encoding"),
        Tag::ENCODED_BASE16 => Some("suggested base16 encoding"),
        Tag::ENCODED_CBOR => Some("encoded cbor data item"),
        Tag::URI => Some("uri"),
        Tag::BASE64URL => Some("base64url encoded text"),
        Tag::BASE64 => Some("base64 encoded text"),
        Tag::REGEX => Some("regex"),
        Tag::MIME => Some("mime message"),
        Tag::UUID => Some("uuid"),
        Tag::NETWORK_ADDRESS => Some("network address"),
        Tag::SELF_DESCRIBE_CBOR => Some("self describe cbor"),
        Tag::EPOCH_DATE => Some("epoch date value"),
        Tag::DATE => Some("standard date string"),
        Tag::SHAREABLE => Some("shareable value"),
        Tag::SHARED_REF => Some("reference to shared value"),
        Tag::IPV4 => Some("ipv4 address and/or prefix"),
        Tag::IPV6 => Some("ipv6 address and/or prefix"),
        _ => None,
    };

    let extra_line = match tag {
        Tag::DATETIME => Some(datetime_epoch(value)),
        Tag::EPOCH_DATETIME => Some(epoch_datetime(value)),
        Tag::POSITIVE_BIGNUM => Some(positive_bignum(value)),
        Tag::NEGATIVE_BIGNUM => Some(negative_bignum(value)),
        Tag::DECIMAL_FRACTION => Some(decimal_fraction(value)),
        Tag::BIGFLOAT => Some(bigfloat(value)),
        Tag::URI => Some(uri(value)),
        Tag::BASE64URL => Some(base64url(value)),
        Tag::BASE64 => Some(base64(value)),
        Tag::ENCODED_CBOR => Some(encoded_cbor(value)),
        Tag::NETWORK_ADDRESS => Some(network_address(value)),
        Tag::UUID => Some(uuid(value)),
        Tag::EPOCH_DATE => Some(epoch_date(value)),
        Tag::DATE => Some(date_epoch(value)),
        Tag::SHAREABLE => {
            let line = format!("reference({})", context.reference_count.separated_string());
            context.reference_count += 1;
            Some(Line::new("", line))
        }
        Tag::SHARED_REF => Some(shared_ref(value, context.reference_count)),
        Tag::IPV4 => Some(ipv4_address_or_prefix(value)),
        Tag::IPV6 => Some(ipv6_address_or_prefix(value)),
        _ => None,
    };

    let encoding = match tag {
        Tag::ENCODED_BASE64URL => Some(Encoding::Base64Url),
        Tag::ENCODED_BASE64 => Some(Encoding::Base64),
        Tag::ENCODED_BASE16 => Some(Encoding::Base16),
        Tag::NETWORK_ADDRESS => Some(Encoding::Base16),
        Tag::UUID => Some(Encoding::Base16),
        _ => context.encoding,
    };

    let comment = if let Some(extra) = extra {
        format!("{}, tag({})", extra, tag.0)
    } else {
        format!("tag({})", tag.0)
    };

    let encoding = std::mem::replace(&mut context.encoding, encoding);
    let sublines = iter::once(Line::from_value(context, value))
        .chain(extra_line)
        .collect();
    context.encoding = encoding;
    Line {
        hex,
        comment,
        sublines,
    }
}

fn datetime_epoch(value: &DataItem) -> Line {
    let date = if let DataItem::TextString(TextString { data, .. }) = value {
        match DateTime::parse_from_rfc3339(data) {
            Ok(value) => value,
            Err(err) => {
                return Line::new("", format!("error parsing datetime: {}", err));
            }
        }
    } else {
        return Line::new("", "invalid type for datetime");
    };

    Line::new("", format!("epoch({})", date.format("%s%.f")))
}

fn epoch_datetime(value: &DataItem) -> Line {
    let date = match *value {
        DataItem::Integer { value, .. } => {
            if value >= (i64::max_value() as u64) {
                None
            } else {
                NaiveDateTime::from_timestamp_opt(value as i64, 0)
            }
        }

        DataItem::Negative { value, .. } => {
            if value >= (i64::max_value() as u64) {
                None
            } else if let Some(value) = (-1i64).checked_sub(value as i64) {
                NaiveDateTime::from_timestamp_opt(value, 0)
            } else {
                None
            }
        }

        DataItem::Float { value, .. } => {
            if value - 1.0 <= (i64::min_value() as f64) || value >= (i64::max_value() as f64) {
                None
            } else {
                let (value, fract) = if value < 0.0 {
                    (value - 1.0, (1.0 + value.fract()) * 1_000_000_000.0)
                } else {
                    (value, value.fract() * 1_000_000_000.0)
                };
                NaiveDateTime::from_timestamp_opt(value as i64, fract as u32)
            }
        }

        DataItem::ByteString(..)
        | DataItem::IndefiniteByteString(..)
        | DataItem::TextString(..)
        | DataItem::IndefiniteTextString(..)
        | DataItem::Array { .. }
        | DataItem::Map { .. }
        | DataItem::Tag { .. }
        | DataItem::Simple(..) => {
            return Line::new("", "invalid type for epoch datetime");
        }
    };

    if let Some(date) = date {
        Line::new("", format!("datetime({})", date.format("%FT%T%.fZ")))
    } else {
        Line::new("", "offset is too large")
    }
}

fn date_epoch(value: &DataItem) -> Line {
    let date = if let DataItem::TextString(TextString { data, .. }) = value {
        match NaiveDate::parse_from_str(data, "%Y-%m-%d") {
            Ok(value) => value,
            Err(err) => {
                return Line::new("", format!("error parsing date: {}", err));
            }
        }
    } else {
        return Line::new("", "invalid type for date");
    };

    Line::new(
        "",
        format!(
            "epoch({})",
            date.signed_duration_since(NaiveDate::from_ymd(1970, 1, 1))
                .num_days()
                .separated_string()
        ),
    )
}

fn epoch_date(value: &DataItem) -> Line {
    let date = match *value {
        DataItem::Integer { value, .. } => i64::try_from(value).ok().and_then(|value| {
            NaiveDate::from_ymd(1970, 1, 1).checked_add_signed(chrono::Duration::days(value))
        }),

        DataItem::Negative { value, .. } => i64::try_from(value)
            .ok()
            .and_then(|value| (-1i64).checked_sub(value))
            .and_then(|value| {
                NaiveDate::from_ymd(1970, 1, 1).checked_add_signed(chrono::Duration::days(value))
            }),

        _ => {
            return Line::new("", "invalid type for epoch date");
        }
    };

    if let Some(date) = date {
        Line::new("", format!("date({})", date.format("%F")))
    } else {
        Line::new("", "offset is too large")
    }
}

fn shared_ref(value: &DataItem, known_references: u64) -> Line {
    match *value {
        DataItem::Integer { value, .. } => {
            if value < known_references {
                Line::new("", format!("reference-to({})", value.separated_string()))
            } else {
                Line::new(
                    "",
                    format!(
                        "reference-to({}), not previously shared",
                        value.separated_string()
                    ),
                )
            }
        }
        _ => Line::new("", "invalid type for shared ref"),
    }
}

fn extract_positive_bignum(value: &DataItem) -> Option<BigUint> {
    if let DataItem::ByteString(ByteString { data, .. }) = value {
        Some(BigUint::from_bytes_be(data))
    } else {
        None
    }
}

fn positive_bignum(value: &DataItem) -> Line {
    extract_positive_bignum(value)
        .map(|num| Line::new("", format!("bignum({})", num)))
        .unwrap_or_else(|| Line::new("", "invalid type for bignum"))
}

fn extract_negative_bignum(value: &DataItem) -> Option<BigInt> {
    if let DataItem::ByteString(ByteString { data, .. }) = value {
        Some(BigInt::from(-1) - BigInt::from_bytes_be(Sign::Plus, data))
    } else {
        None
    }
}

fn negative_bignum(value: &DataItem) -> Line {
    extract_negative_bignum(value)
        .map(|num| Line::new("", format!("bignum({})", num)))
        .unwrap_or_else(|| Line::new("", "invalid type for bignum"))
}

fn extract_fraction(value: &DataItem, base: usize) -> Result<BigRational, &'static str> {
    Ok(match value {
        DataItem::Array { data, .. } => {
            if data.len() != 2 {
                return Err("invalid type");
            }
            let (exponent, positive_exponent) = match data[0] {
                DataItem::Integer { value, .. } => {
                    if value <= usize::max_value() as u64 {
                        (value as usize, true)
                    } else {
                        return Err("exponent is too large");
                    }
                }
                DataItem::Negative { value, .. } => {
                    if value < usize::max_value() as u64 {
                        (value as usize + 1, false)
                    } else {
                        return Err("exponent is too large");
                    }
                }
                _ => return Err("invalid type"),
            };
            let mantissa = match data[1] {
                DataItem::Integer { value, .. } => BigInt::from(value),
                DataItem::Negative { value, .. } => BigInt::from(-1) - BigInt::from(value),
                DataItem::Tag {
                    tag: Tag::POSITIVE_BIGNUM,
                    ref value,
                    ..
                } => match extract_positive_bignum(value) {
                    Some(value) => BigInt::from_biguint(Sign::Plus, value),
                    _ => return Err("invalid type"),
                },
                DataItem::Tag {
                    tag: Tag::NEGATIVE_BIGNUM,
                    ref value,
                    ..
                } => match extract_negative_bignum(value) {
                    Some(value) => value,
                    _ => return Err("invalid type"),
                },
                _ => return Err("invalid type"),
            };
            let multiplier = if positive_exponent {
                Ratio::from_integer(pow(BigInt::from(base), exponent))
            } else {
                Ratio::new(BigInt::from(1), pow(BigInt::from(base), exponent))
            };
            Ratio::from_integer(mantissa) * multiplier
        }
        _ => return Err("invalid type"),
    })
}

fn decimal_fraction(value: &DataItem) -> Line {
    // TODO: https://github.com/rust-num/num-rational/issues/10
    extract_fraction(value, 10)
        .map(|fraction| Line::new("", format!("decimal fraction({})", fraction)))
        .unwrap_or_else(|err| Line::new("", format!("{} for decimal fraction", err)))
}

fn bigfloat(value: &DataItem) -> Line {
    // TODO: https://github.com/rust-num/num-rational/issues/10
    extract_fraction(value, 2)
        .map(|fraction| Line::new("", format!("bigfloat({})", fraction)))
        .unwrap_or_else(|err| Line::new("", format!("{} for bigfloat", err)))
}

fn uri(value: &DataItem) -> Line {
    if let DataItem::TextString(TextString { data, .. }) = value {
        Line::new(
            "",
            if Url::parse(data).is_ok() {
                "valid URL (checked against URL Standard, not RFC 3986)"
            } else {
                "invalid URL (checked against URL Standard, not RFC 3986)"
            },
        )
    } else {
        Line::new("", "invalid type for uri")
    }
}

fn base64_base(
    value: &DataItem,
    encoding: data_encoding::Encoding,
) -> Result<impl Iterator<Item = Line>, String> {
    if let DataItem::TextString(TextString { data, .. }) = value {
        let data = encoding
            .decode(data.as_bytes())
            .map_err(|err| format!("{}", err))?;
        let mut line = Line::new("", "");
        line.sublines.extend(bytes_to_hex(None, &data));
        let merged = line.merge();
        Ok(merged
            .lines()
            .skip(1)
            .map(|line| Line::new("", line.split_at(3).1.replace("#  ", "#")))
            .collect::<Vec<_>>()
            .into_iter())
    } else {
        Err("invalid type".into())
    }
}

fn base64url(value: &DataItem) -> Line {
    base64_base(value, data_encoding::BASE64URL_NOPAD)
        .map(|lines| {
            let mut line = Line::new("", "base64url decoded");
            line.sublines.extend(lines);
            line
        })
        .unwrap_or_else(|err| Line::new("", format!("{} for base64url", err)))
}

fn base64(value: &DataItem) -> Line {
    base64_base(value, data_encoding::BASE64)
        .map(|lines| {
            let mut line = Line::new("", "base64 decoded");
            line.sublines.extend(lines);
            line
        })
        .unwrap_or_else(|err| Line::new("", format!("{} for base64", err)))
}

fn encoded_cbor(value: &DataItem) -> Line {
    if let DataItem::ByteString(ByteString { data, .. }) = value {
        match parse_bytes(data) {
            Ok(value) => {
                let mut line = Line::new("", "encoded cbor data item");
                line.sublines
                    .extend(value.to_hex().lines().map(|line| Line::new("", line)));
                line
            }
            Err(err) => {
                let mut line = Line::new("", "failed to parse encoded cbor data item");
                line.sublines.push(Line::new("", format!("{:?}", err)));
                line
            }
        }
    } else {
        Line::new("", "invalid type for encoded cbor")
    }
}

fn uuid(value: &DataItem) -> Line {
    if let DataItem::ByteString(ByteString { data, .. }) = value {
        if let Ok(uuid) = Uuid::from_slice(data) {
            let version = uuid
                .get_version()
                .map(|v| format!("{:?}", v))
                .unwrap_or_else(|| "Unknown".into());

            let variant = format!("{:?}", uuid.get_variant());

            let uuid_base58 = bs58::encode(uuid.as_bytes()).into_string();
            let uuid_base64 = data_encoding::BASE64_NOPAD.encode(uuid.as_bytes());
            let mut line = Line::new(
                "",
                format!(
                    "uuid(variant({}), version({}, {}))",
                    variant,
                    uuid.get_version_num(),
                    version
                ),
            );
            line.sublines.extend(vec![
                Line::new("", format!("base16({})", uuid)),
                Line::new("", format!("base58({})", uuid_base58)),
                Line::new("", format!("base64({})", uuid_base64)),
            ]);
            line
        } else {
            Line::new("", "invalid data length for uuid")
        }
    } else {
        Line::new("", "invalid type for uuid")
    }
}

fn network_address(value: &DataItem) -> Line {
    if let DataItem::ByteString(ByteString { data, .. }) = value {
        match data.len() {
            4 => {
                let addr = Ipv4Addr::from([data[0], data[1], data[2], data[3]]);
                Line::new("", format!("IPv4 address({})", addr))
            }
            6 => {
                let addr = format!(
                    "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
                    data[0], data[1], data[2], data[3], data[4], data[5]
                );
                Line::new("", format!("MAC address({})", addr))
            }
            16 => {
                let addr = Ipv6Addr::from([
                    data[0], data[1], data[2], data[3], data[4], data[5], data[6], data[7],
                    data[8], data[9], data[10], data[11], data[12], data[13], data[14], data[15],
                ]);
                Line::new("", format!("IPv6 address({})", addr))
            }
            _ => Line::new("", "invalid data length for network address"),
        }
    } else {
        Line::new("", "invalid type for network address")
    }
}

fn ipv4_address_or_prefix(value: &DataItem) -> Line {
    match value {
        DataItem::ByteString(ByteString { data, .. }) => {
            if let Ok(bytes) = <[_; 4]>::try_from(data.as_slice()) {
                Line::new("", format!("IPv4 address({})", Ipv4Addr::from(bytes)))
            } else {
                Line::new("", "invalid data length for IPv4 address")
            }
        }
        DataItem::Array { data, .. } => {
            match data.get(0) {
                Some(DataItem::Integer { value: length, .. }) => {
                    if let Some(DataItem::ByteString(ByteString { data: prefix, .. })) = data.get(1)
                    {
                        if prefix.ends_with(&[0]) {
                            return Line::new("", "invalid prefix, ends with zero byte");
                        }
                        // TODO: check that the defined prefix has all zero bits after the given length
                        // https://www.rfc-editor.org/rfc/rfc9164.html#section-4.3
                        let mut bytes = [0; 4];
                        bytes[..prefix.len()].copy_from_slice(prefix);
                        Line::new(
                            "",
                            format!("IPv4 prefix({}/{})", Ipv4Addr::from(bytes), length),
                        )
                    } else {
                        Line::new("", "invalid type for network address")
                    }
                }
                Some(DataItem::ByteString(ByteString { data: address, .. })) => {
                    let address = if let Ok(address) = <[_; 4]>::try_from(address.as_slice()) {
                        Ipv4Addr::from(address)
                    } else {
                        return Line::new("", "invalid data length for IPv4 address");
                    };
                    let length = match data.get(1) {
                        Some(DataItem::Integer { value, .. }) => Some(value),
                        Some(DataItem::Simple(Simple::NULL)) => None,
                        _ => {
                            return Line::new("", "invalid type for network address");
                        }
                    };
                    let zone = match data.get(2) {
                        Some(DataItem::Integer { value, .. }) => Some(value.to_string()),
                        Some(DataItem::TextString(TextString { data, .. })) => Some(data.clone()),
                        None => None,
                        _ => {
                            return Line::new("", "invalid type for network address");
                        }
                    };
                    match (length, zone) {
                        (Some(length), Some(zone)) => Line::new(
                            "",
                            format!(
                                "IPv4 address-and-zone-and-prefix({}%{}/{})",
                                address, zone, length
                            ),
                        ),
                        (Some(length), None) => Line::new(
                            "",
                            format!("IPv4 address-and-prefix({}/{})", address, length),
                        ),
                        (None, Some(zone)) => {
                            Line::new("", format!("IPv4 address-and-zone({}%{})", address, zone))
                        }
                        (None, None) => Line::new("", "invalid type for network address"),
                    }
                }
                _ => Line::new("", "invalid type for network address"),
            }
        }
        _ => Line::new("", "invalid type for network address"),
    }
}

fn ipv6_address_or_prefix(value: &DataItem) -> Line {
    match value {
        DataItem::ByteString(ByteString { data, .. }) => {
            if let Ok(bytes) = <[_; 16]>::try_from(data.as_slice()) {
                Line::new("", format!("IPv6 address({})", Ipv6Addr::from(bytes)))
            } else {
                Line::new("", "invalid data length for IPv6 address")
            }
        }
        DataItem::Array { data, .. } => {
            match data.get(0) {
                Some(DataItem::Integer { value: length, .. }) => {
                    if let Some(DataItem::ByteString(ByteString { data: prefix, .. })) = data.get(1)
                    {
                        if prefix.ends_with(&[0]) {
                            return Line::new("", "invalid prefix, ends with zero byte");
                        }
                        // TODO: check that the defined prefix has all zero bits after the given length
                        // https://www.rfc-editor.org/rfc/rfc9164.html#section-4.3
                        let mut bytes = [0; 16];
                        bytes[..prefix.len()].copy_from_slice(prefix);
                        Line::new(
                            "",
                            format!("IPv6 prefix({}/{})", Ipv6Addr::from(bytes), length),
                        )
                    } else {
                        Line::new("", "invalid type for network address")
                    }
                }
                Some(DataItem::ByteString(ByteString { data: address, .. })) => {
                    let address = if let Ok(address) = <[_; 16]>::try_from(address.as_slice()) {
                        Ipv6Addr::from(address)
                    } else {
                        return Line::new("", "invalid data length for IPv6 address");
                    };
                    let length = match data.get(1) {
                        Some(DataItem::Integer { value, .. }) => Some(value),
                        Some(DataItem::Simple(Simple::NULL)) => None,
                        _ => {
                            return Line::new("", "invalid type for network address");
                        }
                    };
                    let zone = match data.get(2) {
                        Some(DataItem::Integer { value, .. }) => Some(value.to_string()),
                        Some(DataItem::TextString(TextString { data, .. })) => Some(data.clone()),
                        None => None,
                        _ => {
                            return Line::new("", "invalid type for network address");
                        }
                    };
                    match (length, zone) {
                        (Some(length), Some(zone)) => Line::new(
                            "",
                            format!(
                                "IPv6 address-and-zone-and-prefix({}%{}/{})",
                                address, zone, length
                            ),
                        ),
                        (Some(length), None) => Line::new(
                            "",
                            format!("IPv6 address-and-prefix({}/{})", address, length),
                        ),
                        (None, Some(zone)) => {
                            Line::new("", format!("IPv6 address-and-zone({}%{})", address, zone))
                        }
                        (None, None) => Line::new("", "invalid type for network address"),
                    }
                }
                _ => Line::new("", "invalid type for network address"),
            }
        }
        _ => Line::new("", "invalid type for network address"),
    }
}

fn float_to_hex(value: f64, mut bitwidth: FloatWidth) -> Line {
    if bitwidth == FloatWidth::Unknown {
        bitwidth = FloatWidth::SixtyFour;
    }

    let hex = match bitwidth {
        FloatWidth::Unknown => unreachable!(),
        FloatWidth::Sixteen => format!("f9 {:04x}", f16::from_f64(value).to_bits()),
        FloatWidth::ThirtyTwo => format!("fa {:08x}", (value as f32).to_bits()),
        FloatWidth::SixtyFour => format!("fb {:016x}", value.to_bits()),
    };

    let comment = format!(
        "float({})",
        if value.is_nan() {
            "NaN".to_owned()
        } else if value.is_infinite() {
            if value.is_sign_negative() {
                "-Infinity".to_owned()
            } else {
                "Infinity".to_owned()
            }
        } else {
            value.separated_string()
        }
    );

    Line::new(hex, comment)
}

fn simple_to_hex(simple: Simple) -> Line {
    let Simple(value) = simple;

    let hex = if value < 24 {
        format!("{:02x}", 0b1110_0000 | value)
    } else {
        format!("f8 {:02x}", value)
    };

    let extra = match simple {
        Simple::FALSE => "false, ",
        Simple::TRUE => "true, ",
        Simple::NULL => "null, ",
        Simple::UNDEFINED => "undefined, ",
        Simple(24..=32) => "reserved, ",
        _ => "unassigned, ",
    };

    let comment = format!("{}simple({})", extra, value);

    Line::new(hex, comment)
}

impl DataItem {
    pub fn to_hex(&self) -> String {
        let mut context = Context {
            encoding: None,
            reference_count: 0,
        };
        Line::from_value(&mut context, self).merge()
    }
}
