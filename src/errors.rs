use pyo3::create_exception;
use pyo3::exceptions;

pub const TOO_FEW_DATA_POINTS_MSG: &str =
    "Input data must consist of at least 2 not null data points.";
pub const DISTANCE_TOO_SMALL_MSG: &str =
    "Distance of provided input data is too small for requested desired distance.";
pub const INCONSISTENT_LENGTH_MSG: &str =
    "Input data `coordinates` and `times` lists must be of equal length.";
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
    sportgems,
    InconsistentLengthException,
    exceptions::PyException
);
create_exception!(
    sportgems,
    DistanceTooSmallException,
    exceptions::PyException
);
create_exception!(
    sportgems,
    TooFewDataPointsException,
    exceptions::PyException
);
create_exception!(
    sportgems,
    InvalidDesiredDistanceException,
    exceptions::PyException
);
create_exception!(sportgems, NoSectionFoundException, exceptions::PyException);
