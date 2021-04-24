# sportgems

[![PyPI](https://badge.fury.io/py/sportgems.svg)](https://badge.fury.io/py/sportgems) [![Python](https://img.shields.io/pypi/pyversions/sportgems.svg?style=plastic)](https://badge.fury.io/py/sportgems) [![Build Status](https://github.com/fgebhart/sportgems/workflows/Test/badge.svg)](https://github.com/fgebhart/sportgems/actions?query=workflow%3ATest) [![Deploy Docs](https://github.com/fgebhart/sportgems/actions/workflows/deploy_docs.yml/badge.svg)](https://github.com/fgebhart/sportgems/actions/workflows/deploy_docs.yml)

Sportgems finds valuable gems 💎 in your activities 🚴


## What is it?
Sportgems lets you efficiently parse your activity data. It will search and find your
sections with either max velocity or max climb (see below). It will determine the start,
end and speed of whatever desired sections you are interested in, e.g. 1km, 2km, 10km
and others. 

Sportgems is used in [workoutizer](https://github.com/fgebhart/workoutizer) to find your
fastest 1km (and other 💎) in all your activities and visualize it. See for example this
screenshot of an activity in workoutizer, with the fastest 3km section being highlighted
in yellow:

<img src="https://i.imgur.com/nOYiFm6.png" width="800">

## Installation
Sportgems is written in rust and bundled in a python package using [pyo3](https://pyo3.rs/). Simply
install it using pip:
```
pip install sportgems
```

## Documentation
https://fgebhart.github.io/sportgems/

## How to use it?

In order to search for gems 💎 in your activity, pass a path and desired distance to e.g.
`find_fastest_section_in_fit` like:

```python
from sportgems import find_fastest_section_in_fit

desired_distance = 1_000  # in meter
path_to_fit_file = "tests/data/2019-09-14-17-22-05.fit"
result = find_fastest_section_in_fit(desired_distance, path_to_fit_file)
```
The result will be a python object with the following attributes:
```python
print(f'Found fastest section, from {result.start=} to {result.end=} with {result.velocity=} m/s')
```

which prints:
```
Found fastest section, from result.start=635 to result.end=725 with result.velocity=2.898669803146783 m/s
```

## Changelog
https://fgebhart.github.io/sportgems/changelog/

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
