extern crate pyo3;

mod gem_finder;

use pyo3::prelude::*;
use crate::gem_finder::GemFinder;


#[py::modinit(_sportgems)]
fn init_mod(py: Python, m: &PyModule) -> PyResult<()> {
    #[pyfn(m, "example_fn")]
    fn search(_py: Python, times: Vec<f64>, coordinates: Vec<(f64, f64)>)
    -> PyResult<(u32, u32)> {

        let finder = GemFinder::new(1000, coordinates, times);
        println!("{:?}", finder);

        let result = (1, 2);
        Ok(result)
    }

    Ok(())
}
