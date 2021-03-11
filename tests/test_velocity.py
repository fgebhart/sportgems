from sportgems import find_fastest_section, find_fastest_section_in_fit, DistanceTooSmallException

import pytest


def test_find_fastest_section__synthetic_data(track):
    # search for the fastest 1km (=1000m) with some dummy data
    result = find_fastest_section(1_000, track.times, track.coordinates)
    assert result.start == 131
    assert result.end == 185
    assert int(result.velocity) == 18


def test_find_fastest_section_in_fit(fit_file):
    # test fastest 1km
    # note: values have to be in sync with rust unit test test_find_fastest_section_in_fit
    result = find_fastest_section_in_fit(1_000, fit_file)
    assert result.start == 635
    assert result.end == 725
    assert round(result.velocity, 3) == 2.915
    
    # test fastest 2km
    result = find_fastest_section_in_fit(2_000, fit_file)
    assert result.start == 543
    assert result.end == 820
    assert round(result.velocity, 3) == 2.329
    
    # test fastest 3km
    result = find_fastest_section_in_fit(3_000, fit_file)
    assert result.start == 438
    assert result.end == 945
    assert round(result.velocity, 3) == 2.136
    
    # test fastest 5km
    result = find_fastest_section_in_fit(5_000, fit_file)
    assert result.start == 82
    assert result.end == 1170
    assert round(result.velocity, 3) == 1.844
    
    # test fastest 10km
    with pytest.raises(DistanceTooSmallException, match="Distance of provided input data is too small for requested desired distance."):
        result = find_fastest_section_in_fit(10_000, fit_file)
