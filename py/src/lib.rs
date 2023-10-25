use pyo3::prelude::*;
use xvc_rust::dispatch;
use xvc_rust::error::Error as XvcError;

/// Call Xvc with the command line arguments
#[pyfunction]
fn run(args: Vec<&str>) -> PyResult<()> {
    dispatch(args).map_err(|e| XvcPyError(e).into())
}

struct XvcPyError(XvcError);

impl From<XvcPyError> for PyErr {
    fn from(error: XvcPyError) -> PyErr {
        PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(error.0.to_string())
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn xvc(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(run, m)?)?;
    Ok(())
}
