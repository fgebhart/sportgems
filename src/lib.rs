extern crate pyo3;

pub mod fit_reader;
mod gem_finder;
pub mod geo;

use crate::gem_finder::InputData;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pyclass(name = "FastestSection", dict)]
struct FastestSectionPy {
    #[pyo3(get)]
    pub valid: bool,
    #[pyo3(get)]
    pub start: u32,
    #[pyo3(get)]
    pub end: u32,
    #[pyo3(get)]
    pub velocity: f64,
}

#[pyclass(name = "PowerSection", dict)]
struct PowerSectionPy {
    #[pyo3(get)]
    pub valid: bool,
    #[pyo3(get)]
    pub start: u32,
    #[pyo3(get)]
    pub end: u32,
    #[pyo3(get)]
    pub power: f64,
}

#[pyfunction]
fn find_fastest_section(
    _py: Python,
    fastest_distance: u32,
    times: Vec<f64>,
    coordinates: Vec<(f64, f64)>,
) -> Py<FastestSectionPy> {
    let mut finder = InputData::new(fastest_distance, coordinates, times);
    let result = finder.find_fastest_section();
    let gil = Python::acquire_gil();
    let py = gil.python();
    Py::new(
        py,
        FastestSectionPy {
            valid: result.valid,
            start: result.start,
            end: result.end,
            velocity: result.target_value,
        },
    )
    .unwrap()
}

#[pyfunction]
fn find_best_power_section(
    _py: Python,
    fastest_distance: u32,
    times: Vec<f64>,
    coordinates: Vec<(f64, f64)>,
) -> Py<PowerSectionPy> {
    let mut finder = InputData::new(fastest_distance, coordinates, times);
    let result = finder.find_best_power_section();
    let gil = Python::acquire_gil();
    let py = gil.python();
    Py::new(
        py,
        PowerSectionPy {
            valid: result.valid,
            start: result.start,
            end: result.end,
            power: result.target_value,
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
    let mut finder = InputData::new(fastest_distance, fit_data.coordinates, fit_data.times);
    let result = finder.find_fastest_section();
    let gil = Python::acquire_gil();
    let py = gil.python();
    Py::new(
        py,
        FastestSectionPy {
            valid: result.valid,
            start: result.start,
            end: result.end,
            velocity: result.target_value,
        },
    )
    .unwrap()
}

#[pyclass(name = "FitData", dict)]
struct FitDataPy {
    #[pyo3(get)]
    pub times: Vec<f64>,
    #[pyo3(get)]
    pub coordinates: Vec<(f64, f64)>,
}

#[pyfunction]
fn parse_fit_data(_py: Python, path_to_fit: &str) -> Py<FitDataPy> {
    let fit_data: fit_reader::FitData = fit_reader::parse_fit(path_to_fit);
    let gil = Python::acquire_gil();
    let py = gil.python();
    Py::new(
        py,
        FitDataPy {
            times: fit_data.times,
            coordinates: fit_data.coordinates,
        },
    )
    .unwrap()
}

#[pymodule]
fn sportgems(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(find_fastest_section))?;
    m.add_wrapped(wrap_pyfunction!(find_best_power_section))?;
    m.add_wrapped(wrap_pyfunction!(find_fastest_section_in_fit))?;
    m.add_wrapped(wrap_pyfunction!(parse_fit_data))?;
    Ok(())
}
