use anyhow::anyhow;
use clap::{Parser, ValueEnum};
use std::io::{self, Read, Write};

#[derive(Copy, Clone, Debug, Eq, PartialEq, ValueEnum)]
enum From {
    /// Whichever succeeds first of bytes, hex then diag
    Auto,
    /// Hex-encoded bytes, ignores whitespace and comments from `#` to the end of line
    Hex,
    /// Raw encoded bytes
    Bytes,
    /// Diagnostic notation
    Diag,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, ValueEnum)]
enum To {
    /// Nicely formatted hex with end-of-line annotations describing it
    Annotated,

    /// A simple hex string of the encoded bytes
    Hex,

    /// The raw encoded bytes
    Bytes,

    /// Diagnostic notation with nice whitespace
    Diag,

    /// Diagnostic notation with no additional whitespace
    Compact,

    /// Dump the internal AST representation
    Debug,
}

#[derive(Debug, Parser)]
#[command(version)]
/// A utility for converting between binary, diagnostic, hex and annotated hex
/// formats for CBOR.
struct Args {
    /// What format to attempt to parse the input as
    #[arg(long, default_value_t = From::Auto, value_enum)]
    from: From,

    /// What format to output
    #[arg(long, default_value_t = To::Diag, value_enum)]
    to: To,

    /// Parse a series of undelimited CBOR data items in binary format (a.k.a. the `cbor-seq` data
    /// type).
    #[arg(long, conflicts_with("from"))]
    seq: bool,
}

trait ReadExt: Read {
    fn read_to_vec(&mut self, buffer: &mut Vec<u8>) -> io::Result<bool> {
        let offset = buffer.len();
        buffer.resize(offset + 10 * 1024, 0);
        let len = self.read(&mut buffer[offset..])?;
        buffer.resize(offset + len, 0);
        Ok(len != 0)
    }
}

impl<R: Read> ReadExt for R {}

fn output_item(value: cbor_diag::DataItem, to: To, mut output: impl Write) -> anyhow::Result<()> {
    match to {
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
        To::Debug => {
            write!(output, "{value:#?}")?;
            output.write_all(b"\n")?;
        }
    };

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let input = std::io::stdin();
    let mut input = input.lock();

    let output = std::io::stdout();
    let mut output = output.lock();

    if args.seq {
        let mut data = Default::default();

        while input.read_to_vec(&mut data)? {
            while let Some((value, len)) = cbor_diag::parse_bytes_partial(&data)? {
                output_item(value, args.to, &mut output)?;
                if args.to != To::Bytes && args.to != To::Compact {
                    output.write_all(b"\n")?;
                }
                data.drain(..len);
            }
        }

        if !data.is_empty() {
            return Err(anyhow!("{} bytes remaining after last item", data.len()));
        }
    } else {
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

        output_item(value, args.to, &mut output)?;
    }

    Ok(())
}
