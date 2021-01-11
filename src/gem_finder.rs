use crate::geo;
use crate::test_data;
use crate::test_helper;


#[derive(Debug, Clone)]
pub struct Section {
    pub start_index: u32,
    pub end_index: u32,
    pub distance: f64,
    pub duration: f64,
    pub velocity: f64,
}

pub struct GemFinder {
    pub fastest_distance: u32,
    pub coordinates: Vec<(f64, f64)>,
    pub times: geo::Times,
    pub distances: geo::Distances,
}

impl GemFinder {
    pub fn new(fastest_distance: u32, coordinates: Vec<(f64, f64)>, times: Vec<f64>) -> GemFinder {
        GemFinder {
            fastest_distance,
            coordinates,
            times: geo::Times { values: times },
            distances: geo::Distances { values: vec![] },
        }
    }
    pub fn find_gems(&mut self) -> (bool, u32, u32, f64) {
        assert!(
            self.coordinates.len() == self.times.values.len(),
            "Length of coordinates and times must be equal."
        );
        self.compute_vector_of_distances();
        let total_distance = self.distances.values.last().unwrap().clone();
        if self.fastest_distance as f64 > total_distance {
            return (false, 0, 0, 0.0);
        } else {
            self.search_section()
        }
    }
    pub fn compute_vector_of_distances(&mut self) {
        let mut distance: f64 = 0.0;
        self.distances.values.push(distance);

        // loop through coordinates and calculate the distance from one coordinate to the next one
        for i in 0..self.coordinates.len() - 1 {
            let coordinate = geo::Coordinate {
                lat: self.coordinates[i].0,
                lon: self.coordinates[i].1,
            };
            let next_coordinate = geo::Coordinate {
                lat: self.coordinates[i + 1].0,
                lon: self.coordinates[i + 1].1,
            };
            distance += geo::calculate_distance(coordinate, next_coordinate);
            self.distances.values.push(distance);
        }
    }
    pub fn search_section(&mut self) -> (bool, u32, u32, f64) {
        let mut curr_sec: Section = Section {
            start_index: 0,
            end_index: 0,
            distance: 0.0,
            duration: 0.0,
            velocity: 0.0,
        };
        let mut fastest_sec: Section = Section {
            start_index: 0,
            end_index: 0,
            distance: 0.0,
            duration: 0.0,
            velocity: 0.0,
        };
        while curr_sec.end_index < self.distances.values.len() as u32 - 1 {
            // println!("{:?}", curr_sec);
            if curr_sec.distance < self.fastest_distance as f64 {
                // build up section to have length of fastest_distance
                curr_sec.end_index += 1;
                curr_sec.distance = self.distances.values[curr_sec.end_index as usize]
                    - self.distances.values[curr_sec.start_index as usize]
            } else {
                // update section distance, duration and velocity
                curr_sec.distance = self.distances.values[curr_sec.end_index as usize]
                    - self.distances.values[curr_sec.start_index as usize];
                curr_sec.duration = self.times.values[curr_sec.end_index as usize]
                    - self.times.values[curr_sec.start_index as usize];
                curr_sec.velocity = curr_sec.distance / curr_sec.duration;
                // update fastest section only in case the current
                // distance is not larger than the required distance + 1%
                if curr_sec.distance <= (self.fastest_distance as f64) * 1.01 {
                    if curr_sec.velocity > fastest_sec.velocity {
                        fastest_sec = curr_sec.clone();
                    }
                }
                // now move the start index further, but ensure that start index does not overtake end index
                if curr_sec.start_index < curr_sec.end_index {
                    curr_sec.start_index += 1;
                } else {
                    curr_sec.end_index += 1;
                }
            }
        }
        (
            true,
            fastest_sec.start_index,
            fastest_sec.end_index,
            fastest_sec.velocity,
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_gem_finder_initialization() {
        let mut finder = GemFinder::new(10_000, vec![(48.0, 8.0), (48.0, 8.1)], vec![123.4, 124.6]);
        assert_eq!(finder.fastest_distance, 10_000);
        assert_eq!(finder.coordinates, vec!((48.0, 8.0), (48.0, 8.1)));
        assert_eq!(finder.times.values, vec!(123.4, 124.6));

        // now compute the distances
        finder.compute_vector_of_distances();
        assert_eq!(finder.distances.values, vec!(0.0, 7448.684105664539));

        // test case where fastest distance is greater than the
        // total distance, see above: 10000 > 7448
        let gem = finder.find_gems();
        test_helper::assert_gem_eq(gem, (false, 0, 0, 0.0), 10_000);
    }

    #[test]
    fn test_gem_finder_with_real_activity_data() {
        let fastest_sections = vec![1000, 2000, 3000, 5000, 7500, 10_000, 20_000, 30_000, 50_000, 75_000];

        for section in fastest_sections {
            let data = test_data::get_test_data_a();
            let mut finder = GemFinder::new(section, data.coordinates, data.times);
            let gem = finder.find_gems();

            if section == 1000 {
                test_helper::assert_gem_eq(gem, (true, 10305, 10338, 16.31533731368824), section);
            } else if section == 2000 {
                test_helper::assert_gem_eq(gem, (true, 1390, 1684, 3.948745372703419), section);
            } else if section == 3000 {
                test_helper::assert_gem_eq(gem, (true, 9726, 10336, 2.8213355836114853), section);
            } else if section == 5000 {
                test_helper::assert_gem_eq(gem, (true, 9084, 10346, 2.2944182924032783), section);
            } else if section == 7500 {
                test_helper::assert_gem_eq(gem, (true, 8044, 10358, 1.879004554704313), section);
            } else if section == 10_000 {
                test_helper::assert_gem_eq(gem, (true, 7225, 10335, 1.8593945485699441), section);
            } else if section == 20_000 {
                test_helper::assert_gem_eq(gem, (true, 3004, 10335, 1.5720471824975961), section);
            } else if section == 30_000 {
                test_helper::assert_gem_eq(gem, (true, 2, 11104, 1.5589405108912258), section);
            } else if section == 50_000 {
                test_helper::assert_gem_eq(gem, (false, 0, 0, 0.0), section);
            }

            let data = test_data::get_test_data_b();
            let mut finder = GemFinder::new(section, data.coordinates, data.times);
            let gem = finder.find_gems();
            if section == 1000 {
                test_helper::assert_gem_eq(gem, (true, 12272, 12357, 11.858366762516251), section);
            } else if section == 2000 {
                test_helper::assert_gem_eq(gem, (true, 12253, 12515, 7.595593534279891), section);
            } else if section == 3000 {
                test_helper::assert_gem_eq(gem, (true, 11911, 12482, 5.2618931609244335), section);
            } else if section == 5000 {
                test_helper::assert_gem_eq(gem, (true, 7100, 8135, 4.835730327118072), section);
            } else if section == 7500 {
                test_helper::assert_gem_eq(gem, (true, 7061, 8746, 4.450157590659152), section);
            } else if section == 10_000 {
                test_helper::assert_gem_eq(gem, (true, 7064, 9448, 4.193920762256255), section);
            } else if section == 20_000 {
                test_helper::assert_gem_eq(gem, (true, 7088, 13945, 2.9169924813525343), section);
            } else if section == 30_000 {
                test_helper::assert_gem_eq(gem, (true, 5710, 16325, 2.8257300722244794), section);
            } else if section == 50_000 {
                test_helper::assert_gem_eq(gem, (false, 0, 0, 0.0), section);
            }
        }
    }
}
