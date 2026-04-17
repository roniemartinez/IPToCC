#![allow(deprecated)]

use pyo3::prelude::*;
use pyo3::types::PyString;

/// Looks up the ISO 3166-1 alpha-2 country code for an IPv4 or IPv6 address.
///
/// Accepts either a single address string or an iterable of address strings.
/// For a single string, returns `str | None`. For an iterable, returns a list
/// of `str | None` with one entry per input in order.
#[pyfunction]
fn country_code<'py>(py: Python<'py>, input: &Bound<'py, PyAny>) -> PyResult<Bound<'py, PyAny>> {
    if let Ok(pystr) = input.downcast::<PyString>() {
        return Ok(::iptocc::country_code(pystr.to_str()?).into_pyobject(py)?.into_any());
    }
    let len_hint = input.len().unwrap_or(0);
    let mut results: Vec<Option<&'static str>> = Vec::with_capacity(len_hint);
    for item in input.try_iter()? {
        let item = item?;
        let s = item.downcast::<PyString>()?.to_str()?;
        results.push(::iptocc::country_code(s));
    }
    Ok(results.into_pyobject(py)?.into_any())
}

#[pymodule]
fn iptocc(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(country_code, m)?)?;
    Ok(())
}
