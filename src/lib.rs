extern crate pyo3;

use pyo3::prelude::*;

#[py::modinit(_sportgems)]
fn init_mod(py: Python, m: &PyModule) -> PyResult<()> {
    #[pyfn(m, "example_fn")]
    fn search(_py: Python, times: Vec<f64>, coordinates: Vec<(f64, f64)>) -> PyResult<Vec<f64>> {
        Ok(times)
    }

    Ok(())
}
