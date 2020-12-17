from dataclasses import dataclass

from sportgems import find_gems

import pytest


@pytest.fixture
def track():
    @dataclass
    class Track:
        times: []
        coordinates: []
    
    track = Track(times=[], coordinates=[])
    start_coordinate = (48.0, 8.0)
    # insert a section with 0.001 degree per second
    for i in range(100):
        next_coordinate = (start_coordinate[0], start_coordinate[1] + i * 0.0001)
        track.coordinates.append(next_coordinate)
    # insert a section with 0.003 degree per second which is slightly faster
    for i in range(100):
        next_coordinate = (start_coordinate[0], start_coordinate[1] + i * 0.0003)
        track.coordinates.append(next_coordinate)
    # insert a section with again 0.001 degree per second
    for i in range(100):
        next_coordinate = (start_coordinate[0], start_coordinate[1] + i * 0.0001)
        track.coordinates.append(next_coordinate)
    for i in range(len(track.coordinates)):
        track.times.append(i)
    assert len(track.times) == len(track.coordinates)
    return track


def test_find_gems(track):
    # print(f"times: {track.times}")
    # print(f"coordinates: {track.coordinates}")
    fastest_1km = find_gems(1000, track.times, track.coordinates)
    assert fastest_1km == (99, 112, 77.35165539317262)

