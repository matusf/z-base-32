use pyo3::{create_exception, exceptions::PyException, prelude::*, types::PyBytes};

create_exception!(zbase32, DecodeError, PyException);

#[inline]
#[pyfunction]
#[pyo3(text_signature = "(input: bytes) -> str")]
/// Decode zbase32 encoded string to bytes
fn decode<'a>(py: Python<'a>, input: &'a str) -> PyResult<&'a PyBytes> {
    match crate::decode(input) {
        Ok(b) => Ok(PyBytes::new(py, &b)),
        Err(_) => Err(DecodeError::new_err("Non-zbase32 digit found")),
    }
}

#[inline]
#[pyfunction]
#[pyo3(text_signature = "(input: str) -> bytes")]
/// Encode bytes using a zbase32 and return encoded string
fn encode(input: &[u8]) -> String {
    crate::encode(input)
}

#[pymodule]
fn zbase32(py: Python, m: &PyModule) -> PyResult<()> {
    m.add("DecodeError", py.get_type::<DecodeError>())?;

    m.add_function(wrap_pyfunction!(decode, m)?)?;
    m.add_function(wrap_pyfunction!(encode, m)?)?;
    Ok(())
}
