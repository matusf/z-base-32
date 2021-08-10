#[cfg(test)]
#[macro_use]
extern crate quickcheck;

#[cfg(feature = "python")]
use pyo3::{prelude::*, types::PyBytes};

const ALPHABET: &[u8] = b"ybndrfg8ejkmcpqxot1uwisza345h769";
const INVERSE_ALPHABET: [i8; 123] = [
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    -1, 18, -1, 25, 26, 27, 30, 29, 7, 31, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    -1, 24, 1, 12, 3, 8, 5, 6, 28, 21, 9, 10, -1, 11, 2, 16, 13, 14, 4, 22, 17, 19, -1, 20, 15, 0,
    23,
];

#[cfg_attr(feature = "python", pyfunction)]
pub fn encode(input: &[u8]) -> String {
    let mut result = Vec::new();
    let chunks = input.chunks(5);

    for chunk in chunks {
        let buf = {
            let mut buf = [0u8; 5];
            for (i, &b) in chunk.iter().enumerate() {
                buf[i] = b;
            }
            buf
        };
        result.push(ALPHABET[((buf[0] & 0xF8) >> 3) as usize]);
        result.push(ALPHABET[((buf[0] & 0x07) << 2 | (buf[1] & 0xC0) >> 6) as usize]);
        result.push(ALPHABET[((buf[1] & 0x3E) >> 1) as usize]);
        result.push(ALPHABET[((buf[1] & 0x01) << 4 | (buf[2] & 0xF0) >> 4) as usize]);
        result.push(ALPHABET[((buf[2] & 0x0F) << 1 | (buf[3] & 0x80) >> 7) as usize]);
        result.push(ALPHABET[((buf[3] & 0x7C) >> 2) as usize]);
        result.push(ALPHABET[((buf[3] & 0x03) << 3 | (buf[4] & 0xE0) >> 5) as usize]);
        result.push(ALPHABET[(buf[4] & 0x1F) as usize]);
    }

    let expected_len = (input.len() as f32 * 8.0 / 5.0).ceil() as usize;
    for _ in 0..(result.len() - expected_len) {
        result.pop();
    }
    unsafe { String::from_utf8_unchecked(result) }
}

#[cfg_attr(feature = "python", pyfunction)]
pub fn decode(input: &str) -> Option<Vec<u8>> {
    let mut result = Vec::new();
    for chunk in input.as_bytes().chunks(8) {
        let buf = {
            let mut buf = [0u8; 8];
            for (i, &ch) in chunk.iter().enumerate() {
                match INVERSE_ALPHABET.get(ch as usize) {
                    Some(-1) => return None,
                    Some(x) => buf[i] = *x as u8,
                    None => return None,
                };
            }
            buf
        };
        result.push((buf[0] << 3) | (buf[1] >> 2));
        result.push((buf[1] << 6) | (buf[2] << 1) | (buf[3] >> 4));
        result.push((buf[3] << 4) | (buf[4] >> 1));
        result.push((buf[4] << 7) | (buf[5] << 2) | (buf[6] >> 3));
        result.push((buf[6] << 5) | buf[7]);
    }

    for _ in 0..(result.len() - input.len() * 5 / 8) {
        result.pop();
    }
    Some(result)
}

#[cfg(feature = "python")]
#[pymodule]
fn zbase32(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(encode, m)?)?;
    #[pyfn(m)]
    fn decode<'a>(py: Python<'a>, input: &'a str) -> Option<&'a PyBytes> {
        crate::decode(input).as_ref().map(|b| PyBytes::new(py, b))
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn simple_encode() {
        assert_eq!(encode(b"asdasd"), "cf3seamuco".to_string());
    }

    #[test]
    fn simple_decode() {
        assert_eq!(decode("cf3seamu"), Some(b"asdas".to_vec()))
    }

    #[test]
    fn encode_decode() {
        assert_eq!(decode(&encode(b"foo")).unwrap(), b"foo")
    }

    #[test]
    fn invalid_decode() {
        assert_eq!(decode("bar#"), None)
    }

    quickcheck! {
        fn prop(input: Vec<u8>) -> bool {
            decode(&encode(&input)).unwrap() == input
        }
    }

    quickcheck! {
        fn not_panic(input: String) -> bool {
            decode(&input);
            true
        }
    }
}
