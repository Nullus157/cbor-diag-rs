#![feature(try_blocks, never_type)]

use std::io::{Read, Write};

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
}

#[derive(Debug, structopt::StructOpt)]
#[structopt(name = "cbor-diag", setting = structopt::clap::AppSettings::ColoredHelp)]
/// A utility for converting between binary, diagnostic, hex and annotated hex
/// formats for CBOR.
struct Args {
    /// What format to attempt to parse the input as
    #[structopt(
        long,
        default_value = "auto",
        possible_values(From::variants())
    )]
    from: From,

    /// What format to output
    #[structopt(long, default_value = "diag", possible_values(To::variants()))]
    to: To,
}

trait ResultExt<T, E> {
    fn swap(self) -> Result<E, T>;
}

impl<T, E> ResultExt<T, E> for Result<T, E> {
    fn swap(self) -> Result<E, T> {
        match self {
            Ok(t) => Err(t),
            Err(e) => Ok(e),
        }
    }
}

#[paw::main]
fn main(args: Args) {
    pretty_env_logger::init();

    log::debug!("{:?}", args);

    let result: Result<(), Box<dyn std::error::Error>> = try {
        let input = std::io::stdin();
        let mut input = input.lock();

        let data = {
            let mut data = Default::default();
            input.read_to_end(&mut data)?;
            data
        };

        let value = match args.from {
            From::Auto => {
                #[allow(unreachable_code)] // never type bug
                let result: Result<
                    !,
                    Result<cbor_diag::DataItem, Box<dyn std::error::Error>>,
                > = try {
                    let _ = cbor_diag::parse_bytes(&data).map(Ok).swap()?;
                    let data =
                        String::from_utf8(data).map_err(|e| Err(e.into()))?;
                    let _ = cbor_diag::parse_hex(&data).map(Ok).swap()?;
                    let _ = cbor_diag::parse_diag(&data).map(Ok).swap()?;
                    Err(Err("Failed all parsers".into()))?;
                    unreachable!()
                };
                result.swap()??
            }
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
                Err("not yet implemented")?;
            }
            To::Bytes => {
                Err("not yet implemented")?;
            }
            To::Diag => {
                output.write_all(value.to_diag().as_bytes())?;
                output.write_all(b"\n")?;
            }
        };
    };

    match result {
        Ok(()) => {}
        Err(err) => log::error!("{}", err),
    }
}
