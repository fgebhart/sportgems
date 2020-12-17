from dataclasses import dataclass

from sportgems import find_gems

import pytest


@pytest.fixture
def track():
    @dataclass
    class Track:
        times: list
        coordinates: list
    
    track = Track(times=[], coordinates=[])
    start_coordinate = (48.0, 8.0)
    # insert a section with 0.001 degree per second
    for i in range(100):
        next_coordinate = (start_coordinate[0], start_coordinate[1] + i * 0.0002)
        track.coordinates.append(next_coordinate)
    # insert a section with 0.003 degree per second which is slightly faster
    last_coordinate = track.coordinates[-1]
    for i in range(100):
        next_coordinate = (last_coordinate[0], last_coordinate[1] + i * 0.00025)
        track.coordinates.append(next_coordinate)
    last_coordinate = track.coordinates[-1]
    # insert a section with again 0.001 degree per second
    for i in range(100):
        next_coordinate = (last_coordinate[0], last_coordinate[1] + i * 0.0002)
        track.coordinates.append(next_coordinate)
    for i in range(len(track.coordinates)):
        track.times.append(i)
    assert len(track.times) == len(track.coordinates)
    return track


def test_find_gems(track):
    # search for the fastest 1km (=1000m) with the above created track
    fastest_1km = find_gems(1000, track.times, track.coordinates)
    start_index = fastest_1km[0]
    end_index = fastest_1km[1]
    velocity_found = fastest_1km[2]
    assert start_index == 131
    assert end_index == 184
    assert int(velocity_found) == 18