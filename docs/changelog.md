# Sportgems Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [0.5.0](https://github.com/fgebhart/sportgems/releases/tag/0.5.0) - 2021-04-03

### Changed
* Slightly shift the start index by +1 when computing the distance of a section in
  order to handle target value spikes in situations with poor gps quality.


## [0.4.2](https://github.com/fgebhart/sportgems/releases/tag/0.4.2) - 2021-03-20
### Fixed
* Fill null values in altitude vector and properly treat sections lengths of <=1
  when computing accumulated gained altitude. Also make error message more generic.
  

## [0.4.1](https://github.com/fgebhart/sportgems/releases/tag/0.4.1) - 2021-03-11
### Fixed
* [GH20](https://github.com/fgebhart/sportgems/issues/20): Fix faulty treatment of
  null values in input coordinates. `fill_nans` now applies a forward (and backward)
  filling to fill null values with existing ones.


## [0.4.0](https://github.com/fgebhart/sportgems/releases/tag/0.4.0) - 2021-03-08
### Changed
* introduce custom exceptions
* enable parsing best `climb` section
* remove `valid` attribute (exception is raised if no valid section is found)
* parse `timestamps`, `coordinates`, `altitudes` and `calories` from fit files


## previous releases
see [github releases](https://github.com/fgebhart/sportgems/releases)