from typing import List, Tuple


class FastestSection:
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
        attributes: valid_section, start_index, end_index and velocity
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
        Path to the fit file, which should be parsed by sportgems

    Returns
    -------
    FastestSection
        Returns a Python object of type FastestSection, with the results given as
        attributes: valid_section, start_index, end_index and velocity
    """
    ...