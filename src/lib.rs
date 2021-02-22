extern crate pyo3;

mod gem_finder;
pub mod geo;
pub mod fit_reader;
pub mod test_data;
pub mod test_helper;

use crate::gem_finder::GemFinder;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;


#[pyclass]
struct FastestSection {
    #[pyo3(get)]
    valid_section: bool,
    #[pyo3(get)]
    start_index: u32,
    #[pyo3(get)]
    end_index: u32,
    #[pyo3(get)]
    velocity: f64,
}


#[pyfunction]
fn find_fastest_section(
    _py: Python,
    fastest_distance: u32,
    times: Vec<f64>,
    coordinates: Vec<(f64, f64)>,
) -> Py<FastestSection> {
    let mut finder = GemFinder::new(fastest_distance, coordinates, times);
    let result = finder.find_fastest_section();
    // Ok(result)
    let gil = Python::acquire_gil();
    let py = gil.python();
    Py::new(py, FastestSection {
        valid_section: result.valid_section,
        start_index: result.start_index,
        end_index: result.end_index,
        velocity: result.velocity
    }).unwrap()
}

#[pymodule]
fn sportgems(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(find_fastest_section))?;
    Ok(())
}
