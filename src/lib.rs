extern crate pyo3;

mod gem_finder;
pub mod geo;

use crate::gem_finder::GemFinder;
use pyo3::prelude::*;

#[py::modinit(_sportgems)]
fn init_mod(py: Python, m: &PyModule) -> PyResult<()> {
    #[pyfn(m, "find_gems")]
    fn find_gems(
        _py: Python,
        fastest_distance: u32,
        times: Vec<f64>,
        coordinates: Vec<(f64, f64)>,
    ) -> PyResult<(u32, u32)> {
        let mut finder = GemFinder::new(fastest_distance, coordinates, times);
        let result = finder.find_gems();
        // println!("{:?}", finder);
        Ok(result)
    }

    Ok(())
}
