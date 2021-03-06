from sportgems import find_best_climb_section
from sportgems import find_best_climb_section_in_fit

def test_find_best_climb_section(track):
    # search for the best climb section in 1km (=1000m) with the above created track
    result = find_best_climb_section(1000, track.times, track.coordinates, track.altitudes)
    assert result.valid is True
    assert result.start == 99
    assert result.end == 154
    assert round(result.climb, 3) == 117.818


def test_find_best_climb_section_in_fit(fit_file):
    # test fastest 1km
    # note: values have to be in sync with rust unit test test_find_best_climb_section_in_fit_larger_section
    result = find_best_climb_section_in_fit(1_000, fit_file)
    assert result.valid
    assert result.start == 344
    assert result.end == 586
    assert round(result.climb, 3) == 5.778
    
    # test fastest 2km
    result = find_best_climb_section_in_fit(2_000, fit_file)
    assert result.valid
    assert result.start == 62
    assert result.end == 600
    assert round(result.climb, 3) == 4.975

    # test fastest 3km
    # note: values have to be in sync with rust unit test test_find_best_climb_section_in_fit_larger_section
    result = find_best_climb_section_in_fit(3_000, fit_file)
    assert result.valid
    assert result.start == 63
    assert result.end == 708
    assert round(result.climb, 3) == 3.844
    
    # test fastest 5km
    result = find_best_climb_section_in_fit(5_000, fit_file)
    assert result.valid
    assert result.start == 62
    assert result.end == 1166
    assert round(result.climb, 3) == 2.35
    
    # test fastest 10km
    result = find_best_climb_section_in_fit(10_000, fit_file)
    assert result.valid is False
    assert result.start == 0
    assert result.end == 0
    assert result.climb == 0.0

