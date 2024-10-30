use std::{
    fs::File,
    io::{self, BufRead, Read, Write},
    path::PathBuf,
};

use anyhow::Result;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Decode data
    #[arg(short, long)]
    decode: bool,

    /// File to encode or decode
    file: Option<PathBuf>,

    /// Wrap encoded lines after WRAP character. Use 0 to disable line wrapping
    #[arg(short, long, default_value_t = 76)]
    wrap: usize,
}

fn decode(reader: impl Read, writer: &mut impl Write) -> Result<()> {
    let buf: Result<Vec<String>, std::io::Error> = io::BufReader::new(reader).lines().collect();
    let buf: String = buf?
        .into_iter()
        .map(|mut line| {
            line.truncate(line.trim_end().len());
            line
        })
        .collect();

    writer.write_all(&zbase32::decode(&buf)?)?;
    Ok(())
}

fn encode(reader: &mut impl Read, mut writer: &mut impl Write, wrap: usize) -> Result<()> {
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;
    let buf = zbase32::encode(&buf);
    if wrap > 0 {
        buf.as_bytes()
            .chunks(wrap)
            .map(|chunk| {
                // SAFETY: This is safe since the encoded `buf` string contains only
                // symbols from the basic ASCII set
                let chars = unsafe { std::str::from_utf8_unchecked(chunk) };
                write!(&mut writer, "{}\n", chars)
            })
            .collect::<Result<Vec<()>, _>>()?;
    } else {
        writer.write_all(buf.as_bytes())?;
        writer.write(b"\n")?;
    }
    return Ok(());
}

fn main() -> Result<()> {
    let args = Args::parse();

    if args.decode {
        if let Some(file) = args.file {
            decode(File::open(&file)?, &mut io::stdout())?;
        } else {
            decode(io::stdin().lock(), &mut io::stdout())?;
        };
    } else {
        if let Some(file) = args.file {
            encode(&mut File::open(&file)?, &mut io::stdout(), args.wrap)
        } else {
            encode(&mut io::stdin().lock(), &mut io::stdout(), args.wrap)
        }?;
    };

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::quickcheck;

    quickcheck! {
        fn prop(input: Vec<u8>, wrap: usize) -> bool {
            let mut encoded = Vec::new();
            let mut decoded = Vec::new();
            encode(&mut input.as_slice(), &mut encoded, wrap).unwrap();
            decode(encoded.as_slice(), &mut decoded).unwrap();
            decoded == input
        }
    }
}
