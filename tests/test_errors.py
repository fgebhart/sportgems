from sportgems import (
    find_fastest_section,
    find_best_climb_section,
    DistanceTooSmallException,
    InconsistentLengthException,
    TooFewDataPointsException,
    InvalidDesiredDistanceException,
    NoSectionFoundException,
)

from numpy import NaN
import pytest


TOO_FEW_DATA_POINTS_MSG = "Input data must consist of at least 2 not null data points."
DISTANCE_TOO_SMALL_MSG = "Distance of provided input data is too small for requested desired distance."
INCONSISTENT_LENGTH_MSG = "Input data lists must be of equal length."
INVALID_DESIRED_DISTANCE_MSG = "desired_distance must be greater than 0."
NO_SECTION_FOUND_MSG = "Could not find proper section, check quality of input data or increase tolerance."


def test_find_fastest_section__errors(track):
    # request too large desired distance and expect an exception to be raised
    with pytest.raises(DistanceTooSmallException, match=DISTANCE_TOO_SMALL_MSG):
        find_fastest_section(5_000, track.times, track.coordinates)
    
    # use inconsistent lengths of input lists
    with pytest.raises(InconsistentLengthException, match=INCONSISTENT_LENGTH_MSG):
        find_fastest_section(1_000, [1.0, 2.0, 3.0], [(10.1, 40.2), (10.2, 40.3)])
    
    # use too short input data
    with pytest.raises(TooFewDataPointsException, match=TOO_FEW_DATA_POINTS_MSG):
        find_fastest_section(1_000, [1.0], [(10.3, 42.1)])

    # use Null values for coordinates only
    with pytest.raises(TooFewDataPointsException, match=TOO_FEW_DATA_POINTS_MSG):
        find_fastest_section(1_000, [1.0, 2.0], [(NaN, NaN), (NaN, NaN)])

    # verify that a half Null value for coordinates is also considered null only
    with pytest.raises(TooFewDataPointsException, match=TOO_FEW_DATA_POINTS_MSG):
        find_fastest_section(1_000, [1.0, 2.0], [(NaN, 1.0), (NaN, NaN)])

    # use just on normal values for coordinates
    with pytest.raises(TooFewDataPointsException, match=TOO_FEW_DATA_POINTS_MSG):
        find_fastest_section(1_000, [1.0, 2.0], [(NaN, NaN), (10.2, 40.3)])

    # use Null values for times only
    with pytest.raises(TooFewDataPointsException, match=TOO_FEW_DATA_POINTS_MSG):
        find_fastest_section(1_000, [NaN, NaN], [(10.1, 40.2), (10.2, 40.3)])
    
    # use just on normal values for times
    with pytest.raises(TooFewDataPointsException, match=TOO_FEW_DATA_POINTS_MSG):
        find_fastest_section(1_000, [1.0, NaN], [(10.1, 40.2), (10.2, 40.3)])

    # use too low input data quality by having all data points being equal, and setting desired distance to 0
    with pytest.raises(InvalidDesiredDistanceException, match=INVALID_DESIRED_DISTANCE_MSG):
        find_fastest_section(0, [1., 1., 1., 1.], [(10.3, 42.1), (10.3, 42.1), (10.3, 42.1), (10.3, 42.1)])

    with pytest.raises(TooFewDataPointsException, match=TOO_FEW_DATA_POINTS_MSG):
        find_fastest_section(desired_distance=1, times=[], coordinates=[])
    
    with pytest.raises(TypeError, match="missing required positional argument: desired_distance"):
        find_fastest_section()

    with pytest.raises(TypeError, match="missing required positional argument: times"):
        find_fastest_section(desired_distance=1)
    
    with pytest.raises(TypeError, match="missing required positional argument: coordinates"):
        find_fastest_section(desired_distance=1, times=[])


def test_find_best_climb_section__errors(track):
    # request too large desired distance and expect an exception to be raised
    with pytest.raises(DistanceTooSmallException, match=DISTANCE_TOO_SMALL_MSG):
        find_best_climb_section(5_000, track.times, track.coordinates, track.altitudes)
    
    # use inconsistent lengths of input lists
    with pytest.raises(InconsistentLengthException, match=INCONSISTENT_LENGTH_MSG):
        find_best_climb_section(1_000, [1.0, 2.0, 3.0], [(10.1, 40.2), (10.2, 40.3)], [123.4, 123.2, 345.3])
    
    # use inconsistent lengths of input lists
    with pytest.raises(InconsistentLengthException, match=INCONSISTENT_LENGTH_MSG):
        find_best_climb_section(1_000, [1.0, 2.0, 3.0], [(10.1, 40.2), (10.2, 40.3), (10.3, 40.4)], [123.4, 123.2])
    
    # use too short input data
    with pytest.raises(TooFewDataPointsException, match=TOO_FEW_DATA_POINTS_MSG):
        find_best_climb_section(1_000, [1.0], [(10.3, 42.1)], [123.4])
    
    # use too few normal data points in altitude
    with pytest.raises(TooFewDataPointsException, match=TOO_FEW_DATA_POINTS_MSG):
        find_best_climb_section(1_000, [1.0, 2.0], [(10.3, 42.1), (10.4, 42.2)], [NaN, NaN])
    
    # use too few normal data points in altitude
    with pytest.raises(TooFewDataPointsException, match=TOO_FEW_DATA_POINTS_MSG):
        find_best_climb_section(1_000, [1.0, 2.0], [(10.3, 42.1), (10.4, 42.2)], [123.4, NaN])
    
    # having at least two normal elements in altitude should not raise an TooFewDataPointsException, but NoSectionFoundException
    with pytest.raises(NoSectionFoundException, match=NO_SECTION_FOUND_MSG):
        find_best_climb_section(1_000, [1.0, 2.0], [(10.3, 42.1), (10.4, 42.2)], [123.4, 124.5])

    # use too short input data
    with pytest.raises(TooFewDataPointsException, match=TOO_FEW_DATA_POINTS_MSG):
        find_best_climb_section(1_000, [1.0], [(10.3, 42.1)], [123.4])
    
    # use too low input data quality by having all data points being equal, and setting desired distance to 0
    with pytest.raises(InvalidDesiredDistanceException, match=INVALID_DESIRED_DISTANCE_MSG):
        find_best_climb_section(0, [1., 1., 1., 1.], [(10.3, 42.1), (10.3, 42.1), (10.3, 42.1), (10.3, 42.1)], [123.4, 123.4, 123.4, 123.4])
    
    with pytest.raises(TooFewDataPointsException, match=TOO_FEW_DATA_POINTS_MSG):
        find_best_climb_section(desired_distance=1, times=[], coordinates=[], altitudes=[])
    
    with pytest.raises(TypeError, match="missing required positional argument: desired_distance"):
        find_best_climb_section()

    with pytest.raises(TypeError, match="missing required positional argument: times"):
        find_best_climb_section(desired_distance=1)
    
    with pytest.raises(TypeError, match="missing required positional argument: coordinates"):
        find_best_climb_section(desired_distance=1, times=[])

    with pytest.raises(TypeError, match="missing required positional argument: altitudes"):
        find_best_climb_section(desired_distance=1, times=[], coordinates=[])


