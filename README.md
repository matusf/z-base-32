# z-base-32

![ci](https://github.com/matusf/z-base-32/actions/workflows/ci.yml/badge.svg)

The `z-base-32` is a human-oriented base-32 encoding.

## Rust

### Crate

#### Installation

```sh
cargo add z-base-32
```

#### API

The library exposes two functions with the following signatures and an error type:

```rs
pub fn encode(input: impl AsRef<[u8]>) -> String;

pub fn decode(input: &str) -> Result<Vec<u8>, DecodeError>;

pub struct DecodeError;
```

#### Example

```rs
use zbase32::{encode, decode};

fn main() {
    assert_eq!(encode(b"foo"), "c3zs6".to_string());
    assert_eq!(Ok(b"foo"), decode("c3zs6".to_string()));
    assert_eq!(decode(&encode(b"foo")).unwrap(), b"foo")
}
```

### CLI

This project also provides a CLI utility with a similar interface to the well-known `base64` command.

#### Installation

To install `z-base-32` CLI you can build it from source or download prebuild binary from [releases](https://github.com/matusf/z-base-32/releases/latest).

```console
cargo install --features cli z-base-32
```

#### Example

```console
$ zbase32 -h
z-base-32: human-oriented base-32 encoding

Usage: zbase32 [OPTIONS] [FILE]

Arguments:
  [FILE]  File to encode or decode

Options:
  -d, --decode       Decode data
  -w, --wrap <WRAP>  Wrap encoded lines after COLS character [default: 76]
  -h, --help         Print help
  -V, --version      Print version
```

## Python

### Installation

The `z-base-32` package is published at [PyPI](https://pypi.org/project/z-base-32/). Install it using the following command:

```console
pip install z-base-32
```

### Building

This crate can be compiled with the feature flag `python` in which case it produces Python bindings. To build Python wheels use [`maturin`](https://github.com/PyO3/maturin):

```console
maturin build
```

### API

```py
def encode(input: bytes) -> str:

def decode(input: str) -> bytes:

class DecodeError(Exception):
```

### Example

```py
import zbase32

assert zbase32.encode(b'foo') == 'c3zs6'

assert zbase32.decode('c3zs6') == b'foo'


try:
    zbase32.decode('invalid@char')
except zbase32.DecodeError as e:
    print(e)
```

## References

- <https://philzimmermann.com/docs/human-oriented-base-32-encoding.txt>
