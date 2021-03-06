extern crate pyo3;

pub mod climb;
pub mod dtypes;
pub mod fit_reader;
mod gem_finder;
pub mod math;
pub mod velocity;

use crate::gem_finder::InputData;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pyclass(name = "FastestSection", dict)]
struct PyFastestSection {
    #[pyo3(get)]
    pub valid: bool,
    #[pyo3(get)]
    pub start: u32,
    #[pyo3(get)]
    pub end: u32,
    #[pyo3(get)]
    pub velocity: f64,
}

#[pyclass(name = "ClimbSection", dict)]
struct PyClimbSection {
    #[pyo3(get)]
    pub valid: bool,
    #[pyo3(get)]
    pub start: u32,
    #[pyo3(get)]
    pub end: u32,
    #[pyo3(get)]
    pub climb: f64,
}

#[pyfunction]
fn find_fastest_section(
    _py: Python,
    fastest_distance: u32,
    times: Vec<f64>,
    coordinates: Vec<(f64, f64)>,
) -> Py<PyFastestSection> {
    let mut finder = InputData::new(fastest_distance, coordinates, times, None);
    let result = finder.find_fastest_section();
    let gil = Python::acquire_gil();
    let py = gil.python();
    Py::new(
        py,
        PyFastestSection {
            valid: result.valid,
            start: result.start,
            end: result.end,
            velocity: result.target_value,
        },
    )
    .unwrap()
}

#[pyfunction]
fn find_fastest_section_in_fit(
    _py: Python,
    fastest_distance: u32,
    path_to_fit: &str,
) -> Py<PyFastestSection> {
    let result = fit_reader::find_fastest_section_in_fit(fastest_distance, path_to_fit);
    let gil = Python::acquire_gil();
    let py = gil.python();
    Py::new(
        py,
        PyFastestSection {
            valid: result.valid,
            start: result.start,
            end: result.end,
            velocity: result.target_value,
        },
    )
    .unwrap()
}

#[pyfunction]
fn find_best_climb_section(
    _py: Python,
    fastest_distance: u32,
    times: Vec<f64>,
    coordinates: Vec<(f64, f64)>,
    altitudes: Vec<f64>,
) -> Py<PyClimbSection> {
    let mut finder = InputData::new(fastest_distance, coordinates, times, Some(altitudes));
    let result = finder.find_best_climb_section();
    let gil = Python::acquire_gil();
    let py = gil.python();
    Py::new(
        py,
        PyClimbSection {
            valid: result.valid,
            start: result.start,
            end: result.end,
            climb: result.target_value,
        },
    )
    .unwrap()
}

#[pyfunction]
fn find_best_climb_section_in_fit(
    _py: Python,
    fastest_distance: u32,
    path_to_fit: &str,
) -> Py<PyClimbSection> {
    let result = fit_reader::find_best_climb_section_in_fit(fastest_distance, path_to_fit);
    let gil = Python::acquire_gil();
    let py = gil.python();
    Py::new(
        py,
        PyClimbSection {
            valid: result.valid,
            start: result.start,
            end: result.end,
            climb: result.target_value,
        },
    )
    .unwrap()
}

#[pyclass(name = "FitData", dict)]
struct PyFitData {
    #[pyo3(get)]
    pub calories: u16,
    #[pyo3(get)]
    pub times: Vec<f64>,
    #[pyo3(get)]
    pub coordinates: Vec<(f64, f64)>,
    #[pyo3(get)]
    pub altitudes: Vec<f64>,
}

#[pyfunction]
fn parse_fit_data(_py: Python, path_to_fit: &str) -> Py<PyFitData> {
    let fit_data: fit_reader::FitData = fit_reader::parse_fit(path_to_fit);
    let gil = Python::acquire_gil();
    let py = gil.python();
    Py::new(
        py,
        PyFitData {
            calories: fit_data.calories,
            times: fit_data.times,
            coordinates: fit_data.coordinates,
            altitudes: fit_data.altitudes,
        },
    )
    .unwrap()
}

#[pymodule]
fn sportgems(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(find_fastest_section))?;
    m.add_wrapped(wrap_pyfunction!(find_fastest_section_in_fit))?;
    m.add_wrapped(wrap_pyfunction!(find_best_climb_section))?;
    m.add_wrapped(wrap_pyfunction!(find_best_climb_section_in_fit))?;
    m.add_wrapped(wrap_pyfunction!(parse_fit_data))?;
    Ok(())
}
