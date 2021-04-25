extern crate pyo3;

pub mod climb;
pub mod dtypes;
pub mod exc;
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
        Err(exc::InputDataError::TooFewDataPoints) => Err(
            exc::TooFewDataPointsException::new_err(exc::TOO_FEW_DATA_POINTS_MSG),
        ),
        Err(exc::InputDataError::DistanceTooSmall) => Err(
            exc::DistanceTooSmallException::new_err(exc::DISTANCE_TOO_SMALL_MSG),
        ),
        Err(exc::InputDataError::InconsistentLength) => Err(
            exc::InconsistentLengthException::new_err(exc::INCONSISTENT_LENGTH_MSG),
        ),
        Err(exc::InputDataError::NoSectionFound) => Err(
            exc::NoSectionFoundException::new_err(exc::NO_SECTION_FOUND_MSG),
        ),
        Err(exc::InputDataError::InvalidDesiredDistance) => Err(
            exc::InvalidDesiredDistanceException::new_err(exc::INVALID_DESIRED_DISTANCE_MSG),
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
        Err(exc::InputDataError::TooFewDataPoints) => Err(
            exc::TooFewDataPointsException::new_err(exc::TOO_FEW_DATA_POINTS_MSG),
        ),
        Err(exc::InputDataError::DistanceTooSmall) => Err(
            exc::DistanceTooSmallException::new_err(exc::DISTANCE_TOO_SMALL_MSG),
        ),
        Err(exc::InputDataError::InconsistentLength) => Err(
            exc::InconsistentLengthException::new_err(exc::INCONSISTENT_LENGTH_MSG),
        ),
        Err(exc::InputDataError::NoSectionFound) => Err(
            exc::NoSectionFoundException::new_err(exc::NO_SECTION_FOUND_MSG),
        ),
        Err(exc::InputDataError::InvalidDesiredDistance) => Err(
            exc::InvalidDesiredDistanceException::new_err(exc::INVALID_DESIRED_DISTANCE_MSG),
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
        Err(exc::InputDataError::TooFewDataPoints) => Err(
            exc::TooFewDataPointsException::new_err(exc::TOO_FEW_DATA_POINTS_MSG),
        ),
        Err(exc::InputDataError::DistanceTooSmall) => Err(
            exc::DistanceTooSmallException::new_err(exc::DISTANCE_TOO_SMALL_MSG),
        ),
        Err(exc::InputDataError::InconsistentLength) => Err(
            exc::InconsistentLengthException::new_err(exc::INCONSISTENT_LENGTH_MSG),
        ),
        Err(exc::InputDataError::NoSectionFound) => Err(
            exc::NoSectionFoundException::new_err(exc::NO_SECTION_FOUND_MSG),
        ),
        Err(exc::InputDataError::InvalidDesiredDistance) => Err(
            exc::InvalidDesiredDistanceException::new_err(exc::INVALID_DESIRED_DISTANCE_MSG),
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
        Err(exc::InputDataError::TooFewDataPoints) => Err(
            exc::TooFewDataPointsException::new_err(exc::TOO_FEW_DATA_POINTS_MSG),
        ),
        Err(exc::InputDataError::DistanceTooSmall) => Err(
            exc::DistanceTooSmallException::new_err(exc::DISTANCE_TOO_SMALL_MSG),
        ),
        Err(exc::InputDataError::InconsistentLength) => Err(
            exc::InconsistentLengthException::new_err(exc::INCONSISTENT_LENGTH_MSG),
        ),
        Err(exc::InputDataError::NoSectionFound) => Err(
            exc::NoSectionFoundException::new_err(exc::NO_SECTION_FOUND_MSG),
        ),
        Err(exc::InputDataError::InvalidDesiredDistance) => Err(
            exc::InvalidDesiredDistanceException::new_err(exc::INVALID_DESIRED_DISTANCE_MSG),
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

fn init_exceptions(_py: Python, _module: &PyModule) -> PyResult<()> {
    // module.add("super_useful_constant", "important")
    _module.add(
        "InconsistentLengthException",
        _py.get_type::<exc::InconsistentLengthException>(),
    )?;
    _module.add(
        "TooFewDataPointsException",
        _py.get_type::<exc::TooFewDataPointsException>(),
    )?;
    _module.add(
        "DistanceTooSmallException",
        _py.get_type::<exc::DistanceTooSmallException>(),
    )?;
    _module.add(
        "NoSectionFoundException",
        _py.get_type::<exc::NoSectionFoundException>(),
    )?;
    _module.add(
        "InvalidDesiredDistanceException",
        _py.get_type::<exc::InvalidDesiredDistanceException>(),
    )?;
    Ok(())
}

#[pymodule]
fn sportgems(_py: Python, _module: &PyModule) -> PyResult<()> {
    _module.add_wrapped(wrap_pyfunction!(find_fastest_section))?;
    _module.add_wrapped(wrap_pyfunction!(find_fastest_section_in_fit))?;
    _module.add_wrapped(wrap_pyfunction!(find_best_climb_section))?;
    _module.add_wrapped(wrap_pyfunction!(find_best_climb_section_in_fit))?;
    _module.add_wrapped(wrap_pyfunction!(parse_fit_data))?;
    _module.add_class::<PyFastestSection>()?;
    _module.add_class::<PyClimbSection>()?;
    _module.add_class::<PyFitData>()?;
    
    // add exceptions submodule
    // _module.add_submodule(module: &PyModule)::<PyFitData>()?;
    // _module.add_wrapped(wrap_pymodule!(exceptions))?;

    let exceptions_submodule = PyModule::new(_py, "exc")?;
    init_exceptions(_py, exceptions_submodule)?;
    _module.add_submodule(&exceptions_submodule)?;

    Ok(())
}
