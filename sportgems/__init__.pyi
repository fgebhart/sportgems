from typing import List, Tuple


class FastestSection:
    valid: bool
    start: int
    end: int
    velocity: float

class FitData:
    times: List[float]
    coordinates: List[tuple[float]]

class FitData:
    pass


def find_fastest_section(
    fastest_distance: int, times: List[float], coordinates: List[Tuple[float, float]]
) -> FastestSection:
    """
    Parses the given input coordinates and times to find the fastest section of length
    fastest_distance.

    Parameters
    ----------
    fastest_distance : int
        Length in meter of the fastest section to parse for.

    times : List[float]
        A list of timestamps as floats given in seconds since the Unix epoch, where each
        timestamp corresponds to one coordinate in the coordinates list.

    coordinates : List[Tuple[float, float]]
        A list of tuple of floats, where each tuple represents one coordinate. The first
        float represents the latitude and the second the longitude: (lat, lon).

    Returns
    -------
    FastestSection
        Returns a Python object of type FastestSection, with the results given as
        attributes: valid, start, end and velocity.
    """
    ...


def find_fastest_section_in_fit(
    fastest_distance: int, path_to_fit: str,
) -> FastestSection:
    """
    Takes path to fit file as argument and parses it to find the fastest section of
    length fastest_distance.

    Parameters
    ----------
    fastest_distance : int
        Length in meter of the fastest section to parse for.
    path_to_fit : str
        Path to the fit file, which should be parsed by sportgems.

    Returns
    -------
    FastestSection
        Returns a Python object of type FastestSection, with the results given as
        attributes: valid, start, end and velocity.
    """
    ...


def parse_fit_data(path_to_fit: str) -> FitData:
    """
    Takes path to fit file as argument and parses it. Will return a python object
    with parsed data as attributes.

    Parameters
    ----------
    path_to_fit : str
        Path to the fit file, which should be parsed.

    Returns
    -------
    FitData
        Returns a Python object of type FitData, with the results given as
        attributes: times as list of timestamps and coordinates as list of tuples
        with latitude and longitude: (lat, lon).
    """
    ...