extern crate pyo3;

pub mod climb;
pub mod dtypes;
pub mod errors;
pub mod fit_reader;
mod gem_finder;
pub mod math;
pub mod velocity;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use pyo3::Python;

#[pyclass(name = "FastestSection", dict)]
struct PyFastestSection {
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
    pub start: u32,
    #[pyo3(get)]
    pub end: u32,
    #[pyo3(get)]
    pub climb: f64,
}

#[pyfunction]
fn find_fastest_section(
    _py: Python,
    desired_distance: f64,
    times: Vec<f64>,
    coordinates: Vec<(f64, f64)>,
    tolerance: Option<f64>,
) -> PyResult<Py<PyFastestSection>> {
    let gil = Python::acquire_gil();
    let py = gil.python();
    match velocity::find_fastest_section(desired_distance, coordinates, times, tolerance) {
        Ok(result) => Ok(Py::new(
            py,
            PyFastestSection {
                start: result.start,
                end: result.end,
                velocity: result.target_value,
            },
        )
        .unwrap()),
        Err(errors::InputDataError::TooFewDataPoints) => Err(
            errors::TooFewDataPointsException::new_err(errors::TOO_FEW_DATA_POINTS_MSG),
        ),
        Err(errors::InputDataError::DistanceTooSmall) => Err(
            errors::DistanceTooSmallException::new_err(errors::DISTANCE_TOO_SMALL_MSG),
        ),
        Err(errors::InputDataError::InconsistentLength) => Err(
            errors::InconsistentLengthException::new_err(errors::INCONSISTENT_LENGTH_MSG),
        ),
        Err(errors::InputDataError::NoSectionFound) => Err(
            errors::NoSectionFoundException::new_err(errors::NO_SECTION_FOUND_MSG),
        ),
        Err(errors::InputDataError::InvalidDesiredDistance) => Err(
            errors::InvalidDesiredDistanceException::new_err(errors::INVALID_DESIRED_DISTANCE_MSG),
        ),
    }
}

#[pyfunction]
fn find_fastest_section_in_fit(
    _py: Python,
    desired_distance: f64,
    path_to_fit: &str,
    tolerance: Option<f64>,
) -> PyResult<Py<PyFastestSection>> {
    let gil = Python::acquire_gil();
    let py = gil.python();
    match velocity::find_fastest_section_in_fit(desired_distance, path_to_fit, tolerance) {
        Ok(result) => Ok(Py::new(
            py,
            PyFastestSection {
                start: result.start,
                end: result.end,
                velocity: result.target_value,
            },
        )
        .unwrap()),
        Err(errors::InputDataError::TooFewDataPoints) => Err(
            errors::TooFewDataPointsException::new_err(errors::TOO_FEW_DATA_POINTS_MSG),
        ),
        Err(errors::InputDataError::DistanceTooSmall) => Err(
            errors::DistanceTooSmallException::new_err(errors::DISTANCE_TOO_SMALL_MSG),
        ),
        Err(errors::InputDataError::InconsistentLength) => Err(
            errors::InconsistentLengthException::new_err(errors::INCONSISTENT_LENGTH_MSG),
        ),
        Err(errors::InputDataError::NoSectionFound) => Err(
            errors::NoSectionFoundException::new_err(errors::NO_SECTION_FOUND_MSG),
        ),
        Err(errors::InputDataError::InvalidDesiredDistance) => Err(
            errors::InvalidDesiredDistanceException::new_err(errors::INVALID_DESIRED_DISTANCE_MSG),
        ),
    }
}

#[pyfunction]
fn find_best_climb_section(
    _py: Python,
    desired_distance: f64,
    times: Vec<f64>,
    coordinates: Vec<(f64, f64)>,
    altitudes: Vec<f64>,
    tolerance: Option<f64>,
) -> PyResult<Py<PyClimbSection>> {
    let gil = Python::acquire_gil();
    let py = gil.python();
    match climb::find_best_climb_section(desired_distance, coordinates, times, altitudes, tolerance)
    {
        Ok(result) => Ok(Py::new(
            py,
            PyClimbSection {
                start: result.start,
                end: result.end,
                climb: result.target_value,
            },
        )
        .unwrap()),
        Err(errors::InputDataError::TooFewDataPoints) => Err(
            errors::TooFewDataPointsException::new_err(errors::TOO_FEW_DATA_POINTS_MSG),
        ),
        Err(errors::InputDataError::DistanceTooSmall) => Err(
            errors::DistanceTooSmallException::new_err(errors::DISTANCE_TOO_SMALL_MSG),
        ),
        Err(errors::InputDataError::InconsistentLength) => Err(
            errors::InconsistentLengthException::new_err(errors::INCONSISTENT_LENGTH_MSG),
        ),
        Err(errors::InputDataError::NoSectionFound) => Err(
            errors::NoSectionFoundException::new_err(errors::NO_SECTION_FOUND_MSG),
        ),
        Err(errors::InputDataError::InvalidDesiredDistance) => Err(
            errors::InvalidDesiredDistanceException::new_err(errors::INVALID_DESIRED_DISTANCE_MSG),
        ),
    }
}

#[pyfunction]
fn find_best_climb_section_in_fit(
    _py: Python,
    desired_distance: f64,
    path_to_fit: &str,
    tolerance: Option<f64>,
) -> PyResult<Py<PyClimbSection>> {
    let gil = Python::acquire_gil();
    let py = gil.python();
    match climb::find_best_climb_section_in_fit(desired_distance, path_to_fit, tolerance) {
        Ok(result) => Ok(Py::new(
            py,
            PyClimbSection {
                start: result.start,
                end: result.end,
                climb: result.target_value,
            },
        )
        .unwrap()),
        Err(errors::InputDataError::TooFewDataPoints) => Err(
            errors::TooFewDataPointsException::new_err(errors::TOO_FEW_DATA_POINTS_MSG),
        ),
        Err(errors::InputDataError::DistanceTooSmall) => Err(
            errors::DistanceTooSmallException::new_err(errors::DISTANCE_TOO_SMALL_MSG),
        ),
        Err(errors::InputDataError::InconsistentLength) => Err(
            errors::InconsistentLengthException::new_err(errors::INCONSISTENT_LENGTH_MSG),
        ),
        Err(errors::InputDataError::NoSectionFound) => Err(
            errors::NoSectionFoundException::new_err(errors::NO_SECTION_FOUND_MSG),
        ),
        Err(errors::InputDataError::InvalidDesiredDistance) => Err(
            errors::InvalidDesiredDistanceException::new_err(errors::INVALID_DESIRED_DISTANCE_MSG),
        ),
    }
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
    let gil = Python::acquire_gil();
    let py = gil.python();
    let fit_data: fit_reader::FitData = fit_reader::parse_fit(path_to_fit);
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
    m.add_class::<PyFastestSection>()?;
    m.add_class::<PyClimbSection>()?;
    m.add_class::<PyFitData>()?;
    m.add(
        "InconsistentLengthException",
        _py.get_type::<errors::InconsistentLengthException>(),
    )?;
    m.add(
        "TooFewDataPointsException",
        _py.get_type::<errors::TooFewDataPointsException>(),
    )?;
    m.add(
        "DistanceTooSmallException",
        _py.get_type::<errors::DistanceTooSmallException>(),
    )?;
    m.add(
        "NoSectionFoundException",
        _py.get_type::<errors::NoSectionFoundException>(),
    )?;
    m.add(
        "InvalidDesiredDistanceException",
        _py.get_type::<errors::InvalidDesiredDistanceException>(),
    )?;
    Ok(())
}
