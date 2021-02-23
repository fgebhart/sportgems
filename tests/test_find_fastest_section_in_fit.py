from pathlib import Path

import pytest

from sportgems import find_fastest_section_in_fit


@pytest.fixture
def data_dir():
    return Path(__file__).parent / "data"


@pytest.fixture
def fit_file(data_dir):
    return str(data_dir / "2019-09-14-17-22-05.fit")


def test_find_fastest_section_in_fit(fit_file):
    # test fastest 1km
    result = find_fastest_section_in_fit(1_000, fit_file)
    assert result.valid_section
    assert result.start_index == 635
    assert result.end_index == 725
    assert result.velocity == 2.898669803146783
    
    # test fastest 2km
    result = find_fastest_section_in_fit(2_000, fit_file)
    assert result.valid_section
    assert result.start_index == 544
    assert result.end_index == 822
    assert result.velocity == 2.3154142840217324
    
    # test fastest 5km
    result = find_fastest_section_in_fit(5_000, fit_file)
    assert result.valid_section
    assert result.start_index == 82
    assert result.end_index == 1179
    assert result.velocity == 1.8240325389253973
    
    # test fastest 10km
    result = find_fastest_section_in_fit(10_000, fit_file)
    assert result.valid_section is False
    assert result.start_index == 0
    assert result.end_index == 0
    assert result.velocity == 0.0

