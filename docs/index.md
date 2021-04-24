# Find valuable gems 💎 in your sport 🚴 activity!


`sportgems` is a library for parsing activity data. It can be used to either find the [fastest sections](fastest.md) or the
sections with [best climb value](climb.md).


## Example Usage

In order to search for gems 💎 in your activity, pass a path and desired distance to e.g.
`find_fastest_section_in_fit` like:

``` python
from sportgems import find_fastest_section_in_fit

desired_distance = 1_000  # in meter
path = "tests/data/2019-09-14-17-22-05.fit"
result = find_fastest_section_in_fit(desired_distance, path)

# start and end index of fastest 1000m section:
result.start            # 635
result.end              # 725

# average velocity (in m/s) during that 1000m section:
result.velocity         # 2.898669803146783
```
