from typing import List, Tuple

TOLERANCE = 0.01

class FastestSection:
    valid: bool
    start: int
    end: int
    velocity: float


class ClimbSection:
    valid: bool
    start: int
    end: int
    climb: float


class FitData:
    calories: int
    times: List[float]
    coordinates: List[Tuple[float]]
    altitudes: List[float]


class DistanceTooSmallException(Exception):
    pass


class InconsistentLengthException(Exception):
    pass


class TooFewDataPointsException(Exception):
    pass


def find_fastest_section(
    desired_distance: int, times: List[float], coordinates: List[Tuple[float, float]], tolerance: float = TOLERANCE,
) -> FastestSection:
    """
    Parses the given input coordinates and times to find the fastest section of length
    desired_distance.

    Parameters
    ----------
    desired_distance : int
        Length in meter of the desired fastest section to parse for.

    times : List[float]
        A list of timestamps as floats given in seconds since the Unix epoch, where each
        timestamp corresponds to one coordinate in the coordinates list.

    coordinates : List[Tuple[float, float]]
        A list of tuple of floats, where each tuple represents one coordinate. The first
        float represents the latitude and the second the longitude: (lat, lon).

    tolerance : float
        Percentage value to specify bounds in which the distance of a section is still
        considered to be equal to the desired distance. Because due to the finite
        resolution of activity data, not all sections are exactly e.g. 1000 meter long,
        but with the default of 0.01 a section with 1010 meter will still be considered
        as a 1000 meter section.

    Returns
    -------
    FastestSection
        Returns a Python object of type FastestSection, with the results given as
        attributes: valid, start, end and velocity.
    """
    ...


def find_fastest_section_in_fit(
    desired_distance: int, path_to_fit: str, tolerance: float = TOLERANCE,
) -> FastestSection:
    """
    Takes path to fit file as argument and parses it to find the fastest section of
    length desired_distance.

    Parameters
    ----------
    desired_distance : int
        Length in meter of the desired fastest section to parse for.
    path_to_fit : str
        Path to the fit file, which should be parsed by sportgems.
    tolerance : float
        Percentage value to specify bounds in which the distance of a section is still
        considered to be equal to the desired distance. Because due to the finite
        resolution of activity data, not all sections are exactly e.g. 1000 meter long,
        but with the default of 0.01 a section with 1010 meter will still be considered
        as a 1000 meter section.

    Returns
    -------
    FastestSection
        Returns a Python object of type FastestSection, with the results given as
        attributes: valid, start, end and velocity.
    """
    ...


def find_best_climb_section(
    desired_distance: int, times: List[float], coordinates: List[Tuple[float, float]], altitudes: List[float], tolerance: float = TOLERANCE,
) -> ClimbSection:
    """
    Parses the given input coordinates, times and altitude values to find the section
    with best climb value of length desired_distance. The climb value is determined as
    max climbed uphill meters per time.

    Parameters
    ----------
    desired_distance : int
        Length in meter of the desired best climb section to parse for.
    times : List[float]
        A list of timestamps as floats given in seconds since the Unix epoch, where each
        timestamp corresponds to one coordinate in the coordinates list.
    coordinates : List[Tuple[float, float]]
        A list of tuple of floats, where each tuple represents one coordinate. The first
        float represents the latitude and the second the longitude: (lat, lon).
    altitudes : List[float]
        A list of floats containing the altitude values.
    tolerance : float
        Percentage value to specify bounds in which the distance of a section is still
        considered to be equal to the desired distance. Because due to the finite
        resolution of activity data, not all sections are exactly e.g. 1000 meter long,
        but with the default of 0.01 a section with 1010 meter will still be considered
        as a 1000 meter section.

    Returns
    -------
    ClimbSection
        Returns a Python object of type ClimbSection, with the results given as
        attributes: valid, start, end and climb.
    """
    ...


def find_best_climb_section_in_fit(
    desired_distance: int, path_to_fit: str, tolerance: float = TOLERANCE,
) -> ClimbSection:
    """
    Takes path to fit file as argument and parses it to find the best climb section of
    length desired_distance. The climb value is determined as max climbed uphill
    meters per time.

    Parameters
    ----------
    desired_distance : int
        Length in meter of the desired best climb section to parse for.
    path_to_fit : str
        Path to the fit file, which should be parsed by sportgems.
    tolerance : float
        Percentage value to specify bounds in which the distance of a section is still
        considered to be equal to the desired distance. Because due to the finite
        resolution of activity data, not all sections are exactly e.g. 1000 meter long,
        but with the default of 0.01 a section with 1010 meter will still be considered
        as a 1000 meter section.

    Returns
    -------
    ClimbSection
        Returns a Python object of type ClimbSection, with the results given as
        attributes: valid, start, end and climb.
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
        attributes: times as list of timestamps, coordinates as list of tuples
        with latitude and longitude: (lat, lon), altitudes as list of floats
        and calories as integers.
    """
    ...