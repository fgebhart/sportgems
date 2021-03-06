from sportgems import parse_fit_data


def test_parse_fit_data(fit_file):
    fit_data = parse_fit_data(fit_file)
    assert len(fit_data.coordinates) == len(fit_data.times) == len(fit_data.altitudes)
    assert len(fit_data.coordinates) == 1231
    assert len(fit_data.altitudes) == 1231
    assert fit_data.calories == 432
    assert fit_data.coordinates[100] == (49.40629959106445, 8.695788383483887)
    assert fit_data.times[100] == 1568474841.0
    assert fit_data.altitudes[100] == 254.9
