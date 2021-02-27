from sportgems import find_fastest_section_in_fit


def test_find_fastest_section_in_fit(fit_file):
    # test fastest 1km
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
    
    # test fastest 5km
    result = find_fastest_section_in_fit(5_000, fit_file)
    assert result.valid
    assert result.start == 82
    assert result.end == 1179
    assert round(result.velocity, 3) == 1.824
    
    # test fastest 10km
    result = find_fastest_section_in_fit(10_000, fit_file)
    assert result.valid is False
    assert result.start == 0
    assert result.end == 0
    assert result.velocity == 0.0

