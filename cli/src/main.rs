use anyhow::anyhow;
use std::io::{Read, Write};
use strum::VariantNames;

#[derive(Debug, strum::EnumString, strum::EnumVariantNames)]
#[strum(serialize_all = "snake_case")]
enum From {
    Auto,
    Hex,
    Bytes,
    Diag,
}

#[derive(Debug, strum::EnumString, strum::EnumVariantNames)]
#[strum(serialize_all = "snake_case")]
enum To {
    Annotated,
    Hex,
    Bytes,
    Diag,
    Compact,
}

#[derive(Debug, structopt::StructOpt)]
#[structopt(name = "cbor-diag", setting = structopt::clap::AppSettings::ColoredHelp)]
/// A utility for converting between binary, diagnostic, hex and annotated hex
/// formats for CBOR.
struct Args {
    /// What format to attempt to parse the input as
    #[structopt(long, default_value = "auto", possible_values(From::VARIANTS))]
    from: From,

    /// What format to output
    #[structopt(long, default_value = "diag", possible_values(To::VARIANTS))]
    to: To,
}

#[paw::main]
fn main(args: Args) -> anyhow::Result<()> {
    let input = std::io::stdin();
    let mut input = input.lock();

    let data = {
        let mut data = Default::default();
        input.read_to_end(&mut data)?;
        data
    };

    let value = match args.from {
        From::Auto => cbor_diag::parse_bytes(&data)
            .ok()
            .or_else(|| {
                String::from_utf8(data).ok().and_then(|data| {
                    cbor_diag::parse_hex(&data)
                        .ok()
                        .or_else(|| cbor_diag::parse_diag(&data).ok())
                })
            })
            .ok_or_else(|| anyhow!("Failed all parsers"))?,
        From::Hex => {
            let data = String::from_utf8(data)?;
            cbor_diag::parse_hex(data)?
        }
        From::Bytes => cbor_diag::parse_bytes(data)?,
        From::Diag => {
            let data = String::from_utf8(data)?;
            cbor_diag::parse_diag(data)?
        }
    };

    let output = std::io::stdout();
    let mut output = output.lock();

    match args.to {
        To::Annotated => {
            output.write_all(value.to_hex().as_bytes())?;
        }
        To::Hex => {
            output.write_all(hex::encode(value.to_bytes()).as_bytes())?;
        }
        To::Bytes => {
            output.write_all(&value.to_bytes())?;
        }
        To::Diag => {
            output.write_all(value.to_diag_pretty().as_bytes())?;
            output.write_all(b"\n")?;
        }
        To::Compact => {
            output.write_all(value.to_diag().as_bytes())?;
            output.write_all(b"\n")?;
        }
    };

    Ok(())
}
