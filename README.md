# sportgems

Sportgems finds valuable gems 💎 in your tracked sport 🚴 activity!


## What is it?
Sportgems is a tiny library which lets you efficiently parse your activity data.
It will search and find your fastest sections. It will determine the start,
end and speed of whatever fastest sections you are interested, e.g. 1km, 2km
and 10km. This repo is a tiny rust reincarnation of the
[C++ implementation](https://github.com/fgebhart/sportgems-cpp) of the sportgems algorithm.

Sportgems is (or will be) used in [workoutizer](https://github.com/fgebhart/workoutizer) to
find your fastest 1km (and other 💎) in all your activities and ultimately visualize it.

## Get Started
Sportgems is bundled in a python package using [pyo3](https://pyo3.rs/). Simply
install it with pip:
```bash
pip install sportgems
```

In order to search for gems 💎 in your activity, you need to pass the coordinates as
list of tuples of floats `(lat, lon)` and the timestamps as a list of floats as
seconds since the Unix epoch:
```python
from sportgems import find_gems

fastest_1km = 1000      # in meter
coordinates = [(48.123, 9.35), (48.123, 9.36), (48.123, 9.37), (48.123, 9.38)]
times = [1608228953.8, 1608228954.8, 1608228955.8, 1608228956.8]

result = find_gems(fastest_1km, times, coordinates)
```
The result will be a tuple consisting of `(int, int, float)` where
* the first element is the start index
* the second element is the end index of the fastest section
* the last element is the found velocity
In the above example this would lead to
```python
found_section = result[0]
start_index = result[1]
end_index = result[2]
velocity = result[3]

print(f"Found fastest {int(fastest_1km / 1000)}km: ")
print(f"Fastest section ranges from index {start_index} to {end_index} with a velocity of {velocity}m/s.")
```
which prints
```
The fastest 1km is from index 1 to 2 with a velocity of 743.0908195788583m/s.
```

## How does it work?

The following diagram illustrates how the core algorithm (implemented in `gem_finder.cpp`) works:

<img src="https://i.imgur.com/Jwfyjsk.png" width="500">


## Running the tests

In order to run the rust unit tests simply run
```bash
cargo test
```
To run the python test, which obviously also covers the import of the
python package you first need to install the requirements
```bash
pip install -r requirements.txt
```
and subsequently run the tests
```bash
pytest tests/
```

## Contributing
Contributions are welcome!
