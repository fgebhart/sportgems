extern crate pyo3;

pub mod fit_reader;
mod gem_finder;
pub mod geo;
pub mod test_data;

use crate::gem_finder::GemFinder;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pyclass(name = "FastestSection", dict)]
struct FastestSectionPy {
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
) -> Py<FastestSectionPy> {
    let mut finder = GemFinder::new(fastest_distance, coordinates, times);
    let result = finder.find_fastest_section();
    let gil = Python::acquire_gil();
    let py = gil.python();
    Py::new(
        py,
        FastestSectionPy {
            valid_section: result.valid_section,
            start_index: result.start_index,
            end_index: result.end_index,
            velocity: result.velocity,
        },
    )
    .unwrap()
}

#[pyfunction]
fn find_fastest_section_in_fit(
    _py: Python,
    fastest_distance: u32,
    path_to_fit: &str,
) -> Py<FastestSectionPy> {
    let fit_data: fit_reader::FitData = fit_reader::parse_fit(path_to_fit);
    let mut finder = GemFinder::new(fastest_distance, fit_data.coordinates, fit_data.times);
    let result = finder.find_fastest_section();
    let gil = Python::acquire_gil();
    let py = gil.python();
    Py::new(
        py,
        FastestSectionPy {
            valid_section: result.valid_section,
            start_index: result.start_index,
            end_index: result.end_index,
            velocity: result.velocity,
        },
    )
    .unwrap()
}

#[pymodule]
fn sportgems(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(find_fastest_section))?;
    m.add_wrapped(wrap_pyfunction!(find_fastest_section_in_fit))?;
    Ok(())
}
