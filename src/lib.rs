extern crate pyo3;

use pyo3::prelude::*;

#[py::modinit(_pyrust_example)]
fn init_mod(py: Python, m: &PyModule) -> PyResult<()> {

    #[pyfn(m, "example_fn")]
    fn search(_py: Python, input_str: String) -> PyResult<i32> {
        let length = (input_str.len() as i32) * 2;
        Ok(length)
    }

    Ok(())
}
