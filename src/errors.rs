use pyo3::create_exception;
use pyo3::exceptions;

#[derive(Debug, PartialEq)]
pub enum InputDataError {
    InconsistentLength,
    TooFewDataPoints,
    DistanceTooSmall,
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
