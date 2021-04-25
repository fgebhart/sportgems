from typing import List, Tuple

TOLERANCE = 0.01

class FastestSection:
    """
    Result of parsing activity data for fastest section.

    Attributes:
        start (int): Start index of fastest section.
        end (int): End index of fastest section.
        velocity (float): Found max velocity of given section.
    """
    start: int
    end: int
    velocity: float


class ClimbSection:
    """
    Result of parsing activity data for best climb section.

    Attributes:
        start (int): Start index of best climb section.
        end (int): End index of best climb section.
        climb (float): Found max climb value of given section.
    """
    start: int
    end: int
    climb: float


class FitData:
    """
    Data container returned by e.g. `parse_fit_data` holding the parsed results as attributes.

    Attributes:
        calories (int): Calories parsed from input data given in kcal.
        times (List[float]): List of timestamps since unix epoch.
        coordinates (List[Tuple[float]]): List of tuples of floats containing the parsed coordinates as `(lat, lon)`.
        altitudes (List[float]): List of floats containing the parsed altitude values in meters.
    """
    calories: int
    times: List[float]
    coordinates: List[Tuple[float]]
    altitudes: List[float]


class DistanceTooSmallException(Exception):
    """
    Distance of provided input data is too smaller than the requested
    `desired_distance`. Either descrease `desired_distance` or check
    your input data.
    """
    ...


class InconsistentLengthException(Exception):
    """
    Length of input lists of e.g. `coordinates`, `times` and `altitudes`
    needs to be equal.
    """
    ...


class TooFewDataPointsException(Exception):
    """
    The input data must consist of at least two (not null) data points.
    """
    ...


class NoSectionFoundException(Exception):
    """
    No section with `desired_distance` found, check quality of input data or increase `tolerance`.
    """
    ...


class InvalidDesiredDistanceException(Exception):
    """
    Value of `desired_distance` needs to be greater than zero.
    """
    ...


def find_fastest_section(
    desired_distance: int, times: List[float], coordinates: List[Tuple[float, float]], tolerance: float = TOLERANCE,
) -> FastestSection:
    """
    Parses the given input `coordinates` and `times` to find the fastest section of length
    `desired_distance`.

    Args:
        desired_distance (int): 
            Length in meter of the desired fastest section to parse for.

        times (List[float]):
            A list of timestamps as floats given in seconds since the Unix epoch, where each
            timestamp corresponds to one coordinate in the coordinates list.

        coordinates (List[Tuple[float, float]]):
            A list of tuple of floats, where each tuple represents one coordinate.
            The first float represents the latitude and the second the longitude: (lat, lon).

        tolerance (float): 
            Percentage value to specify bounds in which the distance of a section is still
            considered to be equal to the desired distance. Because due to the finite
            resolution of activity data, not all sections are exactly e.g. 1000 meter long,
            but with the default of 0.01 a section with 1010 meter will still be considered
            as a 1000 meter section.

    Returns:
        FastestSection:
            Returns a Python object of type [`FastestSection`][sportgems.FastestSection], with
            the results given as attributes: `start`, `end` and `velocity`.

    Raises:
        DistanceTooSmallException: If input distance is too small,
            see [`DistanceTooSmallException`][sportgems.DistanceTooSmallException].
        InconsistentLengthException: If length of input lists is not equal,
            see [`InconsistentLengthException`][sportgems.InconsistentLengthException].
        TooFewDataPointsException: If input data has too few data points,
            see [`TooFewDataPointsException`][sportgems.TooFewDataPointsException].
        NoSectionFoundException: If no section with `desired_distance` was found,
            see [`NoSectionFoundException`][sportgems.NoSectionFoundException].
        InvalidDesiredDistanceException: If given `desired_distance` is invalid,
            see [`InvalidDesiredDistanceException`][sportgems.InvalidDesiredDistanceException].
    """
    ...


def find_fastest_section_in_fit(
    desired_distance: int, path_to_fit: str, tolerance: float = TOLERANCE,
) -> FastestSection:
    """
    Takes `path_to_fit` file as argument and parses it to find the fastest section of
    length `desired_distance`.

    Args:
        desired_distance (int):
            Length in meter of the desired fastest section to parse for.
        path_to_fit (str):
            Path to the fit file, which should be parsed by sportgems.
        tolerance (float):
            Percentage value to specify bounds in which the distance of a section is still
            considered to be equal to the desired distance. Because due to the finite
            resolution of activity data, not all sections are exactly e.g. 1000 meter long,
            but with the default of 0.01 a section with 1010 meter will still be considered
            as a 1000 meter section.

    Returns:
        FastestSection:
            Returns a Python object of type [`FastestSection`][sportgems.FastestSection], with
            the results given as attributes: `start`, `end` and `velocity`.

    Raises:
        DistanceTooSmallException: If input distance is too small,
            see [`DistanceTooSmallException`][sportgems.DistanceTooSmallException].
        TooFewDataPointsException: If input data has too few data points,
            see [`TooFewDataPointsException`][sportgems.TooFewDataPointsException].
        NoSectionFoundException: If no section with `desired_distance` was found,
            see [`NoSectionFoundException`][sportgems.NoSectionFoundException].
        InvalidDesiredDistanceException: If given `desired_distance` is invalid,
            see [`InvalidDesiredDistanceException`][sportgems.InvalidDesiredDistanceException].
    """
    ...


def find_best_climb_section(
    desired_distance: int, times: List[float], coordinates: List[Tuple[float, float]], altitudes: List[float], tolerance: float = TOLERANCE,
) -> ClimbSection:
    """
    Parses the given input `coordinates`, `times` and `altitudes` values to find the section
    with best climb value of length `desired_distance`. The climb value is determined as
    max climbed uphill meters per time.

    Args:
        desired_distance (int):
            Length in meter of the desired best climb section to parse for.
        times (List[float]):
            A list of timestamps as floats given in seconds since the Unix epoch, where each
            timestamp corresponds to one coordinate in the coordinates list.
        coordinates (List[Tuple[float, float]]):
            A list of tuple of floats, where each tuple represents one coordinate. The first
            float represents the latitude and the second the longitude: (lat, lon).
        altitudes (List[float]):
            A list of floats containing the altitude values.
        tolerance (float):
            Percentage value to specify bounds in which the distance of a section is still
            considered to be equal to the desired distance. Because due to the finite
            resolution of activity data, not all sections are exactly e.g. 1000 meter long,
            but with the default of 0.01 a section with 1010 meter will still be considered
            as a 1000 meter section.

    Returns:
        ClimbSection:
            Returns a Python object of type [`ClimbSection`][sportgems.ClimbSection], with the
            results given as attributes: `start`, `end` and `climb`.

    Raises:
        DistanceTooSmallException: If input distance is too small,
            see [`DistanceTooSmallException`][sportgems.DistanceTooSmallException].
        InconsistentLengthException: If length of input lists is not equal,
            see [`InconsistentLengthException`][sportgems.InconsistentLengthException].
        TooFewDataPointsException: If input data has too few data points,
            see [`TooFewDataPointsException`][sportgems.TooFewDataPointsException].
        NoSectionFoundException: If no section with `desired_distance` was found,
            see [`NoSectionFoundException`][sportgems.NoSectionFoundException].
        InvalidDesiredDistanceException: If given `desired_distance` is invalid,
            see [`InvalidDesiredDistanceException`][sportgems.InvalidDesiredDistanceException].
    """
    ...


def find_best_climb_section_in_fit(
    desired_distance: int, path_to_fit: str, tolerance: float = TOLERANCE,
) -> ClimbSection:
    """
    Takes `path_to_fit` file as argument and parses it to find the best climb section of
    length `desired_distance`. The climb value is determined as max climbed uphill
    meters per time.

    Args:
        desired_distance (int):
            Length in meter of the desired best climb section to parse for.
        path_to_fit (str):
            Path to the fit file, which should be parsed by sportgems.
        tolerance (float):
            Percentage value to specify bounds in which the distance of a section is still
            considered to be equal to the desired distance. Because due to the finite
            resolution of activity data, not all sections are exactly e.g. 1000 meter long,
            but with the default of 0.01 a section with 1010 meter will still be considered
            as a 1000 meter section.

    Returns:
        ClimbSection:
            Returns a Python object of type [`ClimbSection`][sportgems.ClimbSection], with the
            results given as attributes: `start`, `end` and `climb`.

    Raises:
        DistanceTooSmallException: If input distance is too small,
            see [`DistanceTooSmallException`][sportgems.DistanceTooSmallException].
        TooFewDataPointsException: If input data has too few data points,
            see [`TooFewDataPointsException`][sportgems.TooFewDataPointsException].
        NoSectionFoundException: If no section with `desired_distance` was found,
            see [`NoSectionFoundException`][sportgems.NoSectionFoundException].
        InvalidDesiredDistanceException: If given `desired_distance` is invalid,
            see [`InvalidDesiredDistanceException`][sportgems.InvalidDesiredDistanceException].
    """
    ...

def parse_fit_data(path_to_fit: str) -> FitData:
    """
    Takes `path_to_fit` file as argument and parses it. Will return a python object
    with parsed data as attributes.

    Args:
        path_to_fit (str):
            Path to the fit file, which should be parsed.

    Returns:
        FitData:
            Returns a Python object of type [`FitData`][sportgems.FitData], with the results
            given as attributes.
    """
    ...
    