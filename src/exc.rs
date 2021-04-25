use pyo3::create_exception;
use pyo3::exceptions;

pub const TOO_FEW_DATA_POINTS_MSG: &str =
    "Input data must consist of at least 2 not null data points.";
pub const DISTANCE_TOO_SMALL_MSG: &str =
    "Distance of provided input data is too small for requested desired distance.";
pub const INCONSISTENT_LENGTH_MSG: &str = "Input data lists must be of equal length.";
pub const NO_SECTION_FOUND_MSG: &str =
    "Could not find proper section, check quality of input data or increase tolerance.";
pub const INVALID_DESIRED_DISTANCE_MSG: &str = "desired_distance must be greater than 0.";

#[derive(Debug, PartialEq)]
pub enum InputDataError {
    InconsistentLength,
    TooFewDataPoints,
    DistanceTooSmall,
    NoSectionFound,
    InvalidDesiredDistance,
}

create_exception!(
    exc,
    InconsistentLengthException,
    exceptions::PyException
);
create_exception!(
    exc,
    DistanceTooSmallException,
    exceptions::PyException
);
create_exception!(
    exc,
    TooFewDataPointsException,
    exceptions::PyException
);
create_exception!(
    exc,
    InvalidDesiredDistanceException,
    exceptions::PyException
);
create_exception!(exc, NoSectionFoundException, exceptions::PyException);


// pub fn init_exceptions_submodule(_py: Python, _module: &PyModule) -> PyResult<()> {
//     // m.add_function(wrap_pyfunction!(wkbbytes_with_ids_to_h3, m)?)?;
//     _module.add(
//         "InconsistentLengthException",
//         _py.get_type::<InconsistentLengthException>(),
//     )?;
//     _module.add(
//         "TooFewDataPointsException",
//         _py.get_type::<TooFewDataPointsException>(),
//     )?;
//     _module.add(
//         "DistanceTooSmallException",
//         _py.get_type::<DistanceTooSmallException>(),
//     )?;
//     _module.add(
//         "NoSectionFoundException",
//         _py.get_type::<NoSectionFoundException>(),
//     )?;
//     _module.add(
//         "InvalidDesiredDistanceException",
//         _py.get_type::<InvalidDesiredDistanceException>(),
//     )?;
//     Ok(())
// }
