from sportgems import find_best_climb_section, find_best_climb_section_in_fit, DistanceTooSmallException

import pytest


def test_find_best_climb_section__synthetic_data(track):
    # search for the best climb section in 1km (=1000m) with the above created track
    result = find_best_climb_section(1000, track.times, track.coordinates, track.altitudes)
    assert result.start == 99
    assert result.end == 154
    assert round(result.climb, 3) == 117.818


def test_find_best_climb_section_in_fit(fit_file):
    # test fastest 1km
    # note: values have to be in sync with rust unit test test_find_best_climb_section_in_fit_larger_section
    result = find_best_climb_section_in_fit(1_000, fit_file)
    assert result.start == 346
    assert result.end == 586
    assert round(result.climb, 3) == 5.786
    
    # test fastest 2km
    result = find_best_climb_section_in_fit(2_000, fit_file)
    assert result.start == 56
    assert result.end == 589
    assert round(result.climb, 3) == 5.011

    # test fastest 3km
    # note: values have to be in sync with rust unit test test_find_best_climb_section_in_fit_larger_section
    result = find_best_climb_section_in_fit(3_000, fit_file)
    assert result.start == 63
    assert result.end == 706
    assert round(result.climb, 3) == 3.856
    
    # test fastest 5km
    result = find_best_climb_section_in_fit(5_000, fit_file)
    assert result.start == 61
    assert result.end == 1156
    assert round(result.climb, 3) == 2.368
    
    # test fastest 10km
    with pytest.raises(DistanceTooSmallException, match="Distance of provided input data is too small for requested desired distance."):
        result = find_best_climb_section_in_fit(10_000, fit_file)

