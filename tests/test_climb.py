from sportgems import find_best_climb_section, find_best_climb_section_in_fit
from sportgems import DistanceTooSmallException, InconsistentLengthException, TooFewDataPointsException, InvalidDesiredDistanceException

import pytest


def test_find_best_climb_section__synthetic_data(track):
    # search for the best climb section in 1km (=1000m) with the above created track
    result = find_best_climb_section(1000, track.times, track.coordinates, track.altitudes)
    assert result.start == 99
    assert result.end == 154
    assert round(result.climb, 3) == 117.818


def test_find_fastest_section__errors(track):
    # request too large desired distance and expect an exception to be raised
    with pytest.raises(DistanceTooSmallException, match="Distance of provided input data is too small for requested desired distance."):
        find_best_climb_section(5_000, track.times, track.coordinates, track.altitudes)
    
    # use inconsistent lengths of input lists
    with pytest.raises(InconsistentLengthException, match="Input data `coordinates` and `times` lists must be of equal length."):
        find_best_climb_section(1_000, [1.0, 2.0, 3.0], [(10.1, 40.2), (10.2, 40.3)], [123.4, 123.2, 345.3])
    
    # use too short input data
    with pytest.raises(TooFewDataPointsException, match="Input data must consist of at least 2 data points."):
        find_best_climb_section(1_000, [1.0], [(10.3, 42.1)], [123.4])

    # use too low input data quality by having all data points being equal, and setting desired distance to 0
    with pytest.raises(InvalidDesiredDistanceException, match="desired_distance must be greater than 0."):
        find_best_climb_section(0, [1., 1., 1., 1.], [(10.3, 42.1), (10.3, 42.1), (10.3, 42.1), (10.3, 42.1)], [123.4, 123.4, 123.4, 123.4])
    
    with pytest.raises(TooFewDataPointsException, match="Input data must consist of at least 2 data points."):
        find_best_climb_section(desired_distance=1, times=[], coordinates=[], altitudes=[])
    
    with pytest.raises(TypeError, match="missing required positional argument: desired_distance"):
        find_best_climb_section()

    with pytest.raises(TypeError, match="missing required positional argument: times"):
        find_best_climb_section(desired_distance=1)
    
    with pytest.raises(TypeError, match="missing required positional argument: coordinates"):
        find_best_climb_section(desired_distance=1, times=[])

    with pytest.raises(TypeError, match="missing required positional argument: altitudes"):
        find_best_climb_section(desired_distance=1, times=[], coordinates=[])


def test_find_best_climb_section_in_fit(fit_file):
    # test fastest 1km
    # note: values have to be in sync with rust unit test test_find_best_climb_section_in_fit_larger_section
    result = find_best_climb_section_in_fit(1_000, fit_file)
    assert result.start == 344
    assert result.end == 586
    assert round(result.climb, 3) == 5.778
    
    # test fastest 2km
    result = find_best_climb_section_in_fit(2_000, fit_file)
    assert result.start == 62
    assert result.end == 600
    assert round(result.climb, 3) == 4.975

    # test fastest 3km
    # note: values have to be in sync with rust unit test test_find_best_climb_section_in_fit_larger_section
    result = find_best_climb_section_in_fit(3_000, fit_file)
    assert result.start == 63
    assert result.end == 708
    assert round(result.climb, 3) == 3.844
    
    # test fastest 5km
    result = find_best_climb_section_in_fit(5_000, fit_file)
    assert result.start == 62
    assert result.end == 1166
    assert round(result.climb, 3) == 2.35
    
    # test fastest 10km
    with pytest.raises(DistanceTooSmallException, match="Distance of provided input data is too small for requested desired distance."):
        result = find_best_climb_section_in_fit(10_000, fit_file)

