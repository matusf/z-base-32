# z-base-32

The `z-base-32` is a human oriented base32 encoding.

## API

The library exposes two functions with following signatures and error type:

```rs
pub fn encode(input: &[u8]) -> String;

pub fn decode(input: &str) -> Result<Vec<u8>, DecodeError>;

pub struct DecodeError;
```

### Example

```rs
use zbase32::{encode, decode};

fn main() {
    assert_eq!(encode(b"foo"), "c3zs6".to_string());
    assert_eq!(Ok(b"foo"), decode("c3zs6".to_string()));
    assert_eq!(decode(&encode(b"foo")).unwrap(), b"foo")
}
```

## Python

### Building

This crate can be compiled with feature flag `python` in which case it produces python bindings. To build a Python wheels use [`maturin`](https://github.com/PyO3/maturin):

```console
maturin build --cargo-extra-args="--features python"
```

### API

```py
def encode(input: bytes) -> str:

def decode(input: str) -> bytes:

class DecodeError(Exception):
```

#### Example

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
