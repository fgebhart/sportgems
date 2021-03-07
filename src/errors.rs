use pyo3::create_exception;
use pyo3::exceptions;


pub const TOO_FEW_DATA_POINTS_MSG: &str = "Input data must consist of at least 2 data points.";
pub const DISTANCE_TOO_SMALL_MSG: &str = "Distance of provided input data is too small for requested desired distance.";
pub const INCONSISTENT_LENGTH_MSG: &str = "Input data `coordinates` and `times` lists must be of equal length.";
pub const NO_SECTION_FOUND_MSG: &str = "TODO.";

#[derive(Debug, PartialEq)]
pub enum InputDataError {
    InconsistentLength,
    TooFewDataPoints,
    DistanceTooSmall,
    NoSectionFound,
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
    NoSectionFoundException,
    exceptions::PyException
);
