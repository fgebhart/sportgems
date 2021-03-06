from sportgems import find_fastest_section, find_fastest_section_in_fit
from sportgems import DistanceTooSmallException, InconsistentLengthException, TooFewDataPointsException

import pytest


def test_find_fastest_section_synthetic_data(track):
    # search for the fastest 1km (=1000m) with the above created track
    result = find_fastest_section(1_000, track.times, track.coordinates)
    assert result.valid is True
    assert result.start == 131
    assert result.end == 184
    assert int(result.velocity) == 18


def test_find_fastest_section__errors(track):
    # request too large desired distance and expect an exception to be raised
    with pytest.raises(DistanceTooSmallException, match="Distance of provided input data is too small for requested desired distance."):
        find_fastest_section(5_000, track.times, track.coordinates)
    
    # use inconsistent lengths of input lists
    with pytest.raises(InconsistentLengthException, match="Input data `coordinates` and `times` lists must be of equal length."):
        find_fastest_section(1_000, [1.0, 2.0, 3.0], [(10.1, 40.2), (10.2, 40.3)])
    
    # use too short input data
    with pytest.raises(TooFewDataPointsException, match="Input data must consist of at least 2 data points."):
        find_fastest_section(1_000, [1.0], [(10.3, 42.1)])
    
    with pytest.raises(TooFewDataPointsException, match="Input data must consist of at least 2 data points."):
        find_fastest_section(desired_distance=1, times=[], coordinates=[])
    
    with pytest.raises(TypeError, match="missing required positional argument: desired_distance"):
        find_fastest_section()

    with pytest.raises(TypeError, match="missing required positional argument: times"):
        find_fastest_section(desired_distance=1)
    
    with pytest.raises(TypeError, match="missing required positional argument: coordinates"):
        find_fastest_section(desired_distance=1, times=[])


def test_find_fastest_section_in_fit(fit_file):
    # test fastest 1km
    # note: values have to be in sync with rust unit test test_find_fastest_section_in_fit
    result = find_fastest_section_in_fit(1_000, fit_file)
    assert result.valid
    assert result.start == 635
    assert result.end == 725
    assert round(result.velocity, 3) == 2.899
    
    # test fastest 2km
    result = find_fastest_section_in_fit(2_000, fit_file)
    assert result.valid
    assert result.start == 543
    assert result.end == 821
    assert round(result.velocity, 3) == 2.316
    
    # test fastest 3km
    result = find_fastest_section_in_fit(3_000, fit_file)
    assert result.valid
    assert result.start == 434
    assert result.end == 945
    assert round(result.velocity, 3) == 2.121
    
    # test fastest 5km
    result = find_fastest_section_in_fit(5_000, fit_file)
    assert result.valid
    assert result.start == 82
    assert result.end == 1179
    assert round(result.velocity, 3) == 1.824
    
    # test fastest 10km
    with pytest.raises(DistanceTooSmallException, match="Distance of provided input data is too small for requested desired distance."):
        result = find_fastest_section_in_fit(10_000, fit_file)
