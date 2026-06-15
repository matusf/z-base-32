use std::{
    fmt::Debug,
    fs::File,
    io::{self, BufRead, Read, Write},
    path::PathBuf,
};

use clap::Parser;
use zbase32::DecodeError;

enum Error {
    IoError(io::Error),
    DecodeError,
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::IoError(value)
    }
}

impl From<DecodeError> for Error {
    fn from(_: DecodeError) -> Self {
        Self::DecodeError
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IoError(error) => write!(f, "{error}"),
            Self::DecodeError => write!(f, "Invalid character in input"),
        }
    }
}

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

fn decode(reader: impl Read, writer: &mut impl Write) -> Result<(), Error> {
    let buf: Result<Vec<String>, io::Error> = io::BufReader::new(reader).lines().collect();
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

fn encode(
    reader: &mut impl Read,
    mut writer: &mut impl Write,
    wrap: usize,
) -> Result<(), io::Error> {
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;
    let buf = zbase32::encode(&buf);
    if wrap > 0 {
        buf.as_bytes()
            .chunks(wrap)
            .map(|chunk| {
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

fn main() -> Result<(), Error> {
    let args = Args::parse();

    match (args.decode, args.file) {
        (true, None) => decode(io::stdin().lock(), &mut io::stdout())?,
        (true, Some(file)) => decode(File::open(&file)?, &mut io::stdout())?,
        (false, None) => encode(&mut io::stdin().lock(), &mut io::stdout(), args.wrap)?,
        (false, Some(file)) => encode(&mut File::open(&file)?, &mut io::stdout(), args.wrap)?,
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
