use std::collections::HashSet;

use pyo3::{
    exceptions,
    prelude::*,
    pybacked::PyBackedStr,
    types::{PyBytes, PyList, PyTuple},
    PyAny,
    PyResult,
};
use numpy::{PyArray1, PyReadonlyArray1, PyReadonlyArray2};
use rustc_hash::FxHashMap as HashMap;

use crate::{byte_pair_encode, CoreBPE, Rank};

#[pymethods]
impl CoreBPE {
    #[new]
    fn py_new(
        encoder: HashMap<Vec<u8>, Rank>,
        special_tokens_encoder: HashMap<String, Rank>,
        pattern: &str,
    ) -> PyResult<Self> {
        Self::new_internal(
            encoder,
            special_tokens_encoder,
            pattern,
        )
        .map_err(|e| PyErr::new::<exceptions::PyValueError, _>(e.to_string()))
    }

    // ====================
    // Encoding
    // ====================

    #[pyo3(name = "encode_ordinary")]
    fn py_encode_ordinary(&self, py: Python, text: &str) -> Vec<Rank> {
        py.allow_threads(|| self.encode_ordinary(text))
    }

    #[pyo3(name = "encode_ordinary_list_string_to_buffer")]
    fn py_encode_ordinary_batch_to_buffer(&self, py: Python, texts: Vec<String>, max_len: usize) -> Py<PyAny> {
        let tokens = py.allow_threads(||{
            // Encode each text individually
            let mut encoded: Vec<Vec<Rank>> = texts
                .iter()
                .map(|text| self.encode_ordinary(text))
                .collect();

            // Pad each encoded vector with zeros
            for vec in encoded.iter_mut() {
                vec.resize(max_len, 0);
            }

            // Flatten into a single vector
            let flat_encoded: Vec<Rank> = encoded.into_iter().flatten().collect();

            flat_encoded
        });
        let buffer = TiktokenBuffer { tokens };
        buffer.into_py(py)
    }

    #[pyo3(name = "encode_ordinary_numpy_bytes_to_buffer")]
    fn py_encode_ordinary_numpy_bytes_to_buffer(&self, py: Python, texts: PyReadonlyArray1<'_, PyObject>, max_len: usize) -> Py<PyAny> {
        let texts: Vec<&[u8]> = texts
            .as_slice()
            .unwrap()
            .iter()
            .map(|obj| obj.extract::<&[u8]>(py).unwrap())
            .collect();

        let tokens = py.allow_threads(||{
            // Encode each text individually
            let mut encoded: Vec<Vec<Rank>> = texts
                .iter()
                .map(|text| self.encode_ordinary(std::str::from_utf8(text).unwrap()))
                .collect();

            // Pad each encoded vector with zeros
            for vec in encoded.iter_mut() {
                vec.resize(max_len, 0);
            }

            // Flatten into a single vector
            let flat_encoded: Vec<Rank> = encoded.into_iter().flatten().collect();

            flat_encoded
        });
        let buffer = TiktokenBuffer { tokens };
        buffer.into_py(py)
    }

    #[pyo3(name = "encode_ordinary_numpy_string_to_buffer")]
    fn py_encode_ordinary_numpy_string_to_buffer(&self, py: Python, texts: PyReadonlyArray1<'_, PyObject>, max_len: usize) -> Py<PyAny> {
        let texts: Vec<String> = texts
            .as_slice()
            .unwrap()
            .iter()
            .map(|obj| obj.extract::<String>(py).unwrap())
            .collect();

        let tokens = py.allow_threads(||{
            // Encode each text individually
            let mut encoded: Vec<Vec<Rank>> = texts
                .iter()
                .map(|text| self.encode_ordinary(text))
                .collect();

            // Pad each encoded vector with zeros
            for vec in encoded.iter_mut() {
                vec.resize(max_len, 0);
            }

            // Flatten into a single vector
            let flat_encoded: Vec<Rank> = encoded.into_iter().flatten().collect();

            flat_encoded
        });
        let buffer = TiktokenBuffer { tokens };
        buffer.into_py(py)
    }

    #[pyo3(name = "encode")]
    fn py_encode(
        &self,
        py: Python,
        text: &str,
        allowed_special: HashSet<PyBackedStr>,
    ) -> Vec<Rank> {
        py.allow_threads(|| {
            let allowed_special: HashSet<&str> =
                allowed_special.iter().map(|s| s.as_ref()).collect();
            self.encode(text, &allowed_special).0
        })
    }

    #[pyo3(name = "encode_list_string_to_buffer")]
    fn py_encode_batch_to_buffer(&self, py: Python, texts: Vec<String>, max_len: usize, allowed_special: HashSet<PyBackedStr>) -> Py<PyAny> {
        let tokens = py.allow_threads(||{
            // Get allowed special tokens
            let allowed_special: HashSet<&str> =
                allowed_special.iter().map(|s| s.as_ref()).collect();

            // Encode each text individually
            let mut encoded: Vec<Vec<Rank>> = texts
                .iter()
                .map(|text| self.encode(text, &allowed_special).0)
                .collect();

            // Pad each encoded vector with zeros
            for vec in encoded.iter_mut() {
                vec.resize(max_len, 0);
            }

            // Flatten into a single vector
            let flat_encoded: Vec<Rank> = encoded.into_iter().flatten().collect();

            flat_encoded
        });
        let buffer = TiktokenBuffer { tokens };
        buffer.into_py(py)
    }

    #[pyo3(name = "encode_numpy_bytes_to_buffer")]
    fn py_encode_numpy_bytes_to_buffer(&self, py: Python, texts: PyReadonlyArray1<'_, PyObject>, max_len: usize, allowed_special: HashSet<PyBackedStr>) -> Py<PyAny> {
        let texts: Vec<&[u8]> = texts
            .as_slice()
            .unwrap()
            .iter()
            .map(|obj| obj.extract::<&[u8]>(py).unwrap())
            .collect();

        let tokens = py.allow_threads(||{
            // Get allowed special tokens
            let allowed_special: HashSet<&str> =
                allowed_special.iter().map(|s| s.as_ref()).collect();

            // Encode each text individually
            let mut encoded: Vec<Vec<Rank>> = texts
                .iter()
                .map(|text| self.encode(std::str::from_utf8(text).unwrap(), &allowed_special).0)
                .collect();

            // Pad each encoded vector with zeros
            for vec in encoded.iter_mut() {
                vec.resize(max_len, 0);
            }

            // Flatten into a single vector
            let flat_encoded: Vec<Rank> = encoded.into_iter().flatten().collect();

            flat_encoded
        });
        let buffer = TiktokenBuffer { tokens };
        buffer.into_py(py)
    }

    #[pyo3(name = "encode_numpy_string_to_buffer")]
    fn py_encode_numpy_string_to_buffer(&self, py: Python, texts: PyReadonlyArray1<'_, PyObject>, max_len: usize, allowed_special: HashSet<PyBackedStr>) -> Py<PyAny> {
        let texts: Vec<String> = texts
            .as_slice()
            .unwrap()
            .iter()
            .map(|obj| obj.extract::<String>(py).unwrap())
            .collect();

        let tokens = py.allow_threads(||{
            // Get allowed special tokens
            let allowed_special: HashSet<&str> =
                allowed_special.iter().map(|s| s.as_ref()).collect();

            // Encode each text individually
            let mut encoded: Vec<Vec<Rank>> = texts
                .iter()
                .map(|text| self.encode(text, &allowed_special).0)
                .collect();

            // Pad each encoded vector with zeros
            for vec in encoded.iter_mut() {
                vec.resize(max_len, 0);
            }

            // Flatten into a single vector
            let flat_encoded: Vec<Rank> = encoded.into_iter().flatten().collect();

            flat_encoded
        });
        let buffer = TiktokenBuffer { tokens };
        buffer.into_py(py)
    }

    fn encode_to_tiktoken_buffer(
        &self,
        py: Python,
        text: &str,
        allowed_special: HashSet<PyBackedStr>,
    ) -> Py<PyAny> {
        let tokens = py.allow_threads(|| {
            let allowed_special: HashSet<&str> =
                allowed_special.iter().map(|s| s.as_ref()).collect();
            self.encode(text, &allowed_special).0
        });
        let buffer = TiktokenBuffer { tokens };
        buffer.into_py(py)
    }

    fn _encode_bytes(&self, py: Python, bytes: &[u8]) -> Vec<Rank> {
        py.allow_threads(|| {
            match std::str::from_utf8(bytes) {
                // Straightforward case
                Ok(text) => self.encode_ordinary(text),
                // Oops, don't actually have UTF-8. But we need to do the regex splitting in
                // Unicode space, so we make our best guess at where we would have splits
                Err(e) => {
                    let text = unsafe { std::str::from_utf8_unchecked(&bytes[..e.valid_up_to()]) };
                    let (tokens, last_piece_token_len) = self.encode(text, &HashSet::new());
                    let (mut tokens, last_piece_token_len) =
                        self._increase_last_piece_token_len(tokens, last_piece_token_len);

                    let mut unstable_bytes;
                    if !tokens.is_empty() && last_piece_token_len > 0 {
                        // Lop off the tokens from the last piece and run BPE on the remaining bytes
                        // This likely matches what models see better, e.g. if you assume we're
                        // dealing with truncated UTF-8 bytes.
                        // Niche, but note this may not be correct if we'd have had a regex
                        // split between the valid UTF-8 and the invalid bytes.
                        unstable_bytes = self
                            .decode_bytes(&tokens[tokens.len() - last_piece_token_len..])
                            .unwrap();
                        unstable_bytes.extend_from_slice(&bytes[e.valid_up_to()..]);

                        tokens.truncate(tokens.len() - last_piece_token_len);
                    } else {
                        unstable_bytes = bytes[e.valid_up_to()..].to_vec();
                    }

                    if !unstable_bytes.is_empty() {
                        match self.encoder.get(&unstable_bytes) {
                            Some(token) => tokens.push(*token),
                            None => {
                                tokens.extend(&byte_pair_encode(&unstable_bytes, &self.encoder))
                            }
                        }
                    }
                    tokens
                }
            }
        })
    }

    #[pyo3(name = "encode_with_unstable")]
    fn py_encode_with_unstable(
        &self,
        py: Python,
        text: &str,
        allowed_special: HashSet<PyBackedStr>,
    ) -> Py<PyTuple> {
        let (tokens, completions) = py.allow_threads(|| {
            let allowed_special: HashSet<&str> =
                allowed_special.iter().map(|s| s.as_ref()).collect();
            self._encode_unstable_native(text, &allowed_special)
        });
        let py_completions = PyList::new_bound(
            py,
            completions
                .iter()
                .map(|seq| PyList::new_bound(py, &seq[..])),
        );
        (tokens, py_completions).into_py(py)
    }

    fn encode_single_token(&self, piece: &[u8]) -> PyResult<Rank> {
        if let Some(token) = self.encoder.get(piece).copied() {
            return Ok(token);
        }
        if let Ok(piece_str) = std::str::from_utf8(piece) {
            if let Some(token) = self.special_tokens_encoder.get(piece_str).copied() {
                return Ok(token);
            }
        }
        Err(PyErr::new::<exceptions::PyKeyError, _>(piece.to_owned()))
    }

    fn encode_single_piece(&self, piece: &[u8]) -> Vec<Rank> {
        if let Some(token) = self.encoder.get(piece) {
            return vec![*token];
        }
        byte_pair_encode(piece, &self.encoder)
    }

    // ====================
    // Decoding
    // ====================

    #[pyo3(name = "decode_bytes")]
    fn py_decode_bytes(&self, py: Python, tokens: Vec<Rank>) -> Result<Py<PyBytes>, PyErr> {
        match py.allow_threads(|| self.decode_bytes(&tokens)) {
            Ok(bytes) => Ok(PyBytes::new_bound(py, &bytes).into()),
            Err(e) => Err(pyo3::exceptions::PyKeyError::new_err(format!("{}", e))),
        }
    }

    #[pyo3(name = "decode_strip_bytes")]
    fn py_decode_strip_bytes(&self, py: Python, mut tokens: Vec<Rank>) -> Result<Py<PyBytes>, PyErr> {
        if let Some(pos) = tokens.iter().rposition(|&x| x != 0) {
            tokens.truncate(pos + 1);
        } else {
            tokens.clear();
        }

        match py.allow_threads(|| self.decode_bytes(&tokens)) {
            Ok(bytes) => Ok(PyBytes::new_bound(py, &bytes).into()),
            Err(e) => Err(pyo3::exceptions::PyKeyError::new_err(format!("{}", e))),
        }
    }

    #[pyo3(name = "decode_strip_string")]
    fn py_decode_strip_string(&self, py: Python, mut tokens: Vec<Rank>) -> Result<String, PyErr> {
        if let Some(pos) = tokens.iter().rposition(|&x| x != 0) {
            tokens.truncate(pos + 1);
        } else {
            tokens.clear();
        }

        match py.allow_threads(|| self.decode_bytes(&tokens)) {
            Ok(bytes) => String::from_utf8(bytes).map_err(|e| pyo3::exceptions::PyUnicodeDecodeError::new_err(format!("{}", e))),
            Err(e) => Err(pyo3::exceptions::PyKeyError::new_err(format!("{}", e))),
        }
    }

    #[pyo3(name = "decode_batch_string")]
    fn py_decode_batch_string(&self, py: Python, array: PyReadonlyArray2<'_, Rank>) -> Result<Py<PyAny>, PyErr> {
        let tokens_2d = array.as_array();

        let result: Result<Vec<String>, PyErr> = py.allow_threads(||tokens_2d
            .rows()
            .into_iter()
            .map(|row| {
                let row_vec = row.to_vec();
                let truncated_row = if let Some(pos) = row_vec.iter().rposition(|&x| x != 0) {
                    &row_vec[..=pos]
                } else {
                    &[]
                };
                self.decode_bytes(truncated_row)
                .map_err(|e| pyo3::exceptions::PyKeyError::new_err(format!("{}", e)))
                .and_then(|bytes| {
                    String::from_utf8(bytes)
                        .map_err(|e| pyo3::exceptions::PyUnicodeDecodeError::new_err(format!("{}", e)))
                })
            })
            .collect());

            Ok(PyList::new_bound(py, result?).into_py(py))
    }


    fn decode_single_token_bytes(&self, py: Python, token: Rank) -> PyResult<Py<PyBytes>> {
        if let Some(bytes) = self.decoder.get(&token) {
            return Ok(PyBytes::new_bound(py, bytes).into());
        }
        if let Some(bytes) = self.special_tokens_decoder.get(&token) {
            return Ok(PyBytes::new_bound(py, bytes).into());
        }
        Err(PyErr::new::<exceptions::PyKeyError, _>(token.to_string()))
    }

    // ====================
    // Miscellaneous
    // ====================

    fn token_byte_values(&self, py: Python) -> Vec<Py<PyBytes>> {
        self.sorted_token_bytes
            .iter()
            .map(|x| PyBytes::new_bound(py, x).into())
            .collect()
    }
}

#[pyclass]
struct TiktokenBuffer {
    tokens: Vec<Rank>,
}

#[pymethods]
impl TiktokenBuffer {
    // Based on https://github.com/PyO3/pyo3/blob/v0.22.2/tests/test_buffer_protocol.rs#L25
    unsafe fn __getbuffer__(
        slf: Bound<'_, Self>,
        view: *mut pyo3::ffi::Py_buffer,
        flags: std::os::raw::c_int,
    ) -> PyResult<()> {
        if view.is_null() {
            return Err(pyo3::exceptions::PyBufferError::new_err("View is null"));
        }
        if (flags & pyo3::ffi::PyBUF_WRITABLE) == pyo3::ffi::PyBUF_WRITABLE {
            return Err(pyo3::exceptions::PyBufferError::new_err(
                "Object is not writable",
            ));
        }

        (*view).obj = slf.clone().into_any().into_ptr();

        let data = &slf.borrow().tokens;
        (*view).buf = data.as_ptr() as *mut std::os::raw::c_void;
        (*view).len = (data.len() * std::mem::size_of::<Rank>()) as isize;
        (*view).readonly = 1;
        (*view).itemsize = std::mem::size_of::<Rank>() as isize;
        (*view).format = if (flags & pyo3::ffi::PyBUF_FORMAT) == pyo3::ffi::PyBUF_FORMAT {
            let msg = std::ffi::CString::new("I").unwrap();
            msg.into_raw()
        } else {
            std::ptr::null_mut()
        };
        (*view).ndim = 1;
        (*view).shape = if (flags & pyo3::ffi::PyBUF_ND) == pyo3::ffi::PyBUF_ND {
            &mut (*view).len
        } else {
            std::ptr::null_mut()
        };
        (*view).strides = if (flags & pyo3::ffi::PyBUF_STRIDES) == pyo3::ffi::PyBUF_STRIDES {
            &mut (*view).itemsize
        } else {
            std::ptr::null_mut()
        };
        (*view).suboffsets = std::ptr::null_mut();
        (*view).internal = std::ptr::null_mut();

        Ok(())
    }

    unsafe fn __releasebuffer__(&self, view: *mut pyo3::ffi::Py_buffer) {
        std::mem::drop(std::ffi::CString::from_raw((*view).format));
    }
}

#[pymodule]
fn _tiktoken(_py: Python, m: &Bound<PyModule>) -> PyResult<()> {
    m.add_class::<CoreBPE>()?;
    Ok(())
}
