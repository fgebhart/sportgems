# Sportgems Changelog

## sportgems 0.4.1 (2021-03-11)
### Bugfixes
* GH20: Fix faulty treatment of null values in input coordinates. `fill_nans` now
        applies a forward (and backward) filling to fill null values with existing
        ones.


## sportgems 0.4.0 (2021-03-08)
### Changes
* introduce custom exceptions
* enable parsing best `climb` section
* remove `valid` attribute (exception is raised if no valid section is found)
* parse `timestamps`, `coordinates`, `altitudes` and `calories` from fit files


## previous releases
see [github releases](https://github.com/fgebhart/sportgems/releases)