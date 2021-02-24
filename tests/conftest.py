from pathlib import Path

import pytest


@pytest.fixture
def data_dir():
    return Path(__file__).parent / "data"


@pytest.fixture
def fit_file(data_dir):
    return str(data_dir / "2019-09-14-17-22-05.fit")