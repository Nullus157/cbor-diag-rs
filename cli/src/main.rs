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

fn try_main(args: Args) -> Result<(), Box<dyn std::error::Error>> {
    log::debug!("{:?}", args);

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
            .ok_or_else(|| "Failed all parsers")?,
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
            return Err("not yet implemented".into());
        }
        To::Bytes => {
            return Err("not yet implemented".into());
        }
        To::Diag => {
            output.write_all(value.to_diag().as_bytes())?;
            output.write_all(b"\n")?;
        }
    };

    Ok(())
}

#[paw::main]
fn main(args: Args) {
    pretty_env_logger::init();

    match try_main(args) {
        Ok(()) => {}
        Err(err) => log::error!("{}", err),
    }
}
