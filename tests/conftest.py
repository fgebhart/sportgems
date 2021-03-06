from dataclasses import dataclass
from pathlib import Path
from typing import List

import pytest


@dataclass
class Track:
    times: List[float]
    coordinates: List[float]
    altitudes: List[float]


@pytest.fixture
def track():
    def _get_track(length: int = 100):
        # generate synthetic data
        track = Track(times=[], coordinates=[], altitudes=[])
        start_coordinate = (48.0, 8.0)
        start_altitude = 243.0

        # insert a section with 0.0002 degree and +1 altitude per second
        for i in range(length):
            coordinate = (start_coordinate[0], start_coordinate[1] + i * 0.0002)
            track.coordinates.append(coordinate)
            track.altitudes.append(start_altitude + 1 + i)

        # insert a section with 0.00025 degree and +2 altitude per second which is slightly faster and steeper
        last_coordinate = track.coordinates[-1]
        for i in range(length):
            coordinate = (last_coordinate[0], last_coordinate[1] + i * 0.00025)
            track.coordinates.append(coordinate)
            track.altitudes.append(track.altitudes[-1] + 2)

        # insert a section with again 0.0002 degree and -1 altitude per second
        last_coordinate = track.coordinates[-1]
        for i in range(length):
            coordinate = (last_coordinate[0], last_coordinate[1] + i * 0.0002)
            track.coordinates.append(coordinate)
            # insert downwards altitude data
            track.altitudes.append(track.altitudes[-1] - 1)
    
        # insert timestamps as seconds with length equal to coordinates
        for i in range(len(track.coordinates)):
            track.times.append(i)

        assert len(track.times) == len(track.coordinates)
        return track
    return _get_track()


@pytest.fixture
def data_dir():
    return Path(__file__).parent / "data"


@pytest.fixture
def fit_file(data_dir):
    return str(data_dir / "2019-09-14-17-22-05.fit")