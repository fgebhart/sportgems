extern crate pyo3;

mod gem_finder;
pub mod geo;

use crate::gem_finder::GemFinder;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pyfunction]
fn find_gems(
    _py: Python,
    fastest_distance: u32,
    times: Vec<f64>,
    coordinates: Vec<(f64, f64)>,
) -> PyResult<(u32, u32, f64)> {
    let mut finder = GemFinder::new(fastest_distance, coordinates, times);
    let result = finder.find_gems();
    Ok(result)
}

#[pymodule]
fn _sportgems(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(find_gems))?;

    Ok(())
}
