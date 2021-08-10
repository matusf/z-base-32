# z-base-32

The `z-base-32` is a human oriented base32 encoding.

## API

The library exposes two functions with following signatures:

```rs
pub fn encode(input: &[u8]) -> String

pub fn decode(input: &str) -> Option<Vec<u8>>
```

## References

- <https://philzimmermann.com/docs/human-oriented-base-32-encoding.txt>
