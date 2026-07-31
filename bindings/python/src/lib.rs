use std::sync::OnceLock;

use pyo3::prelude::*;
use pyo3::types::{PyList, PyString, PyTuple};

/// One slot per pair of ASCII uppercase letters. The data generator guarantees
/// every country code is exactly that, so this indexes without hashing.
const CC_SLOTS: usize = 26 * 26;

/// Interned `str` objects for the country codes, built once at module import.
///
/// Without this, every hit allocated a fresh two-character `PyUnicode`; a lookup
/// that costs single-digit nanoseconds in Rust should not be paying for an object
/// allocation to report its answer. Returning a cached object is one incref.
static CC_CACHE: OnceLock<Box<[Option<Py<PyString>>; CC_SLOTS]>> = OnceLock::new();

#[inline]
fn cc_slot(cc: &str) -> usize {
    let b = cc.as_bytes();
    (b[0] - b'A') as usize * 26 + (b[1] - b'A') as usize
}

#[inline]
fn to_py<'py>(py: Python<'py>, cc: Option<&'static str>) -> Bound<'py, PyAny> {
    match cc {
        Some(cc) => match CC_CACHE.get().and_then(|cache| cache[cc_slot(cc)].as_ref()) {
            Some(cached) => cached.bind(py).clone().into_any(),
            // Unreachable while the cache is built from `country_code_set`, but
            // a fresh string is a correct answer either way.
            None => PyString::new(py, cc).into_any(),
        },
        None => py.None().into_bound(py),
    }
}

#[inline]
fn lookup_item<'py>(py: Python<'py>, item: &Bound<'py, PyAny>) -> PyResult<Bound<'py, PyAny>> {
    let s = item.cast::<PyString>()?.to_str()?;
    Ok(to_py(py, ::iptocc::country_code(s)))
}

/// Looks up the ISO 3166-1 alpha-2 country code for an IPv4 or IPv6 address.
///
/// Accepts either a single address string or an iterable of address strings.
/// For a single string, returns `str | None`. For an iterable, returns a list
/// of `str | None` with one entry per input in order.
#[pyfunction]
fn country_code<'py>(py: Python<'py>, input: &Bound<'py, PyAny>) -> PyResult<Bound<'py, PyAny>> {
    if let Ok(pystr) = input.cast::<PyString>() {
        return Ok(to_py(py, ::iptocc::country_code(pystr.to_str()?)));
    }

    // Lists and tuples are the overwhelmingly common batch shapes and can be
    // indexed directly, skipping the iterator protocol's per-item call. Exact
    // types only: a subclass may override __iter__, and the slow path honours it.
    if let Ok(list) = input.cast_exact::<PyList>() {
        let mut results = Vec::with_capacity(list.len());
        for item in list.iter() {
            results.push(lookup_item(py, &item)?);
        }
        return Ok(PyList::new(py, results)?.into_any());
    }
    if let Ok(tuple) = input.cast_exact::<PyTuple>() {
        let mut results = Vec::with_capacity(tuple.len());
        for item in tuple.iter() {
            results.push(lookup_item(py, &item)?);
        }
        return Ok(PyList::new(py, results)?.into_any());
    }

    let mut results = Vec::with_capacity(input.len().unwrap_or(0));
    for item in input.try_iter()? {
        results.push(lookup_item(py, &item?)?);
    }
    Ok(PyList::new(py, results)?.into_any())
}

#[pymodule]
fn iptocc(m: &Bound<'_, PyModule>) -> PyResult<()> {
    let py = m.py();
    CC_CACHE.get_or_init(|| {
        let mut cache: Box<[Option<Py<PyString>>; CC_SLOTS]> = Box::new(core::array::from_fn(|_| None));
        for cc in ::iptocc::country_code_set() {
            let slot = &mut cache[cc_slot(cc)];
            if slot.is_none() {
                *slot = Some(PyString::intern(py, cc).unbind());
            }
        }
        cache
    });
    m.add_function(wrap_pyfunction!(country_code, m)?)?;
    Ok(())
}
