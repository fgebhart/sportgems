# sportgems

Sportgems finds valuable gems 💎 in your tracked sport 🚴 activity!


## What is it?
Sportgems lets you efficiently parse your activity data. It will search and find your
fastest sections. It will determine the start, end and speed of whatever fastest sections
you are interested in, e.g. 1km, 2km, 10km and others. This repo is a rust reincarnation of
the [C++ implementation](https://github.com/fgebhart/sportgems-cpp) of the sportgems algorithm.

Sportgems is used in [workoutizer](https://github.com/fgebhart/workoutizer) to
find your fastest 1km (and other 💎) in all your activities and ultimately visualize it.

## Get Started
Sportgems is bundled in a python package using [pyo3](https://pyo3.rs/). Simply
install it using pip:
```bash
pip install sportgems
```

In order to search for gems 💎 in your activity, pass the coordinates as list of tuples of
floats `(lat, lon)` and the timestamps as a list of floats as seconds since the Unix epoch:
```python
from sportgems import find_fastest_section_in_fit

desired_distance = 1_000  # in meter
path_to_fit_file = "tests/data/2019-09-14-17-22-05.fit"
result = find_fastest_section(desired_distance, path_to_fit_file)
```
The result will be a python object with the following attributes:
```python
print(f'Found fastest section, from {result.start=} to {result.end=} with {result.velocity=} m/s')
```
which prints:
```
Found fastest section, from result.start=635 to result.end=725 with result.velocity=2.898669803146783 m/s`
```

## How does it work?

The following diagram illustrates how the core algorithm (implemented in `gem_finder.cpp`) works:

<img src="https://i.imgur.com/Jwfyjsk.png" width="500">

## Running the tests

In order to run the rust unit tests simply run
```
cargo test --no-default-features
```
To run the python tests, you first need to install the requirements
```
pip install -r requirements.txt
```
and subsequently run the tests
```
pytest tests/
```

## Contributing
Contributions are welcome!
