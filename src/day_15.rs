use std::ops::RangeInclusive;
use regex::Regex;
use crate::map::map::Map;
use crate::map::point::Point;


pub fn main() {
    let cave = Cave::new(include_str!("resources/day_15_cave.txt"));
    println!("There are {} beacons that can't be present at y=2000000", cave.count_places_without_beacon_in_row(2000000));
    let (x, y) = cave.find_unreachable_beacon(0..=4_000_000);
    println!("The tuning frequency of unreachable beacon is {}", x*4000000 + y);
}

struct Cave {
    map: Map<Point, Point>,
}

impl Cave {
    fn new(input: &str) -> Cave {
        let mut map = Map::new();
        input.lines().map(|line| {
            let regex = Regex::new("Sensor at x=(?P<sx>-*\\d+), y=(?P<sy>-*\\d+): closest beacon is at x=(?P<bx>-*\\d+), y=(?P<by>-*\\d+)").unwrap();
            let r = regex.captures(line).unwrap();
            (r.name("sx").unwrap().as_str().parse::<isize>().unwrap(),
             r.name("sy").unwrap().as_str().parse::<isize>().unwrap(),
             r.name("bx").unwrap().as_str().parse::<isize>().unwrap(),
             r.name("by").unwrap().as_str().parse::<isize>().unwrap())
        }).for_each(|(sx, sy, bx, by)| {
            map.push(Point::new(sx, sy), Point::new(bx, by))
        });

        Cave { map }
    }

    fn get_ranges_without_beacon_in_row(&self, y: isize) -> Vec<(isize, isize)> {
        let mut ranges = vec![];
        for (sensor, beacon) in self.map.map.clone() {
            let dispersion = (sensor.manhattan_distance(&beacon) as isize) - (y - sensor.get_y()).abs();
            if dispersion < 0 {
                continue;
            }
            let x1 = sensor.get_x() - dispersion;
            let x2 = sensor.get_x() + dispersion;
            ranges.push((x1.min(x2), x1.max(x2)));
        }

        let mut result_ranges: Vec<(isize, isize)> = vec![];
        for (x1, x2) in ranges.clone().iter_mut() {

            for (rx1, rx2) in result_ranges.clone().iter().cloned() {
                if rx1 >= *x1 && rx1 <= *x2 && rx2 >= *x1 && rx2 <= *x2 {
                    // inserted is in between incoming
                    result_ranges.remove(result_ranges.iter().position(|(r1, r2)| *r1 == rx1 && *r2 == rx2).unwrap());
                } else if *x1 >= rx1 && *x1 <= rx2 && *x2 >= rx1 && *x2 <= rx2 {
                    // incoming is inbetween inserted
                    *x1 = rx1;
                    *x2 = rx2;
                    result_ranges.remove(result_ranges.iter().position(|(r1, r2)| *r1 == rx1 && *r2 == rx2).unwrap());
                } else if rx1 >= *x1 && rx1 <= *x2 {
                    // incoming is overlapping inserted from left
                    *x2 = rx2;
                    result_ranges.remove(result_ranges.iter().position(|(r1, r2)| *r1 == rx1 && *r2 == rx2).unwrap());
                } else if rx2 >= *x1 && rx2 <= *x2 {
                    // incoming is overlapping inserted from right
                    *x1 = rx1;
                    result_ranges.remove(result_ranges.iter().position(|(r1, r2)| *r1 == rx1 && *r2 == rx2).unwrap());
                }
                // inserted and incoming are not overlapping each other
            }

            result_ranges.push((*x1, *x2));
        }

        result_ranges
    }

    fn find_unreachable_beacon(&self, range: RangeInclusive<isize>) -> (isize, isize) {
        for y in range.clone() {
            let result_ranges = self.get_ranges_without_beacon_in_row(y);
            for (x1, x2) in result_ranges {
                if x1 - 1 >= *range.start() && x1 - 1 <= *range.end() {
                    return (x1 - 1, y)
                } else if x2 + 1 >= *range.start() && x2 + 1 <= *range.end() {
                    return (x2 + 1, y)
                }
            }
        }

        panic!();
    }

    fn count_places_without_beacon_in_row(&self, y: isize) -> usize {
        let result_ranges = self.get_ranges_without_beacon_in_row(y);

        let mut result = 0;
        for (x1, x2) in &result_ranges {
            result += x2 - x1 + 1
        }

        let devices = self.map.map.clone();
        let mut beacons = devices.values().cloned().collect::<Vec<Point>>();
        beacons.sort();
        beacons.dedup();
        let beacon_count = beacons.iter().filter(|beacon| beacon.get_y() == y).filter(|beacon| {
            result_ranges.iter().find(|(x1, x2)| beacon.get_x() >= *x1 && beacon.get_x() <= *x2).is_some()
        }).count() as isize;

        let sensor_count = devices.keys().cloned().filter(|sensor| sensor.get_y() == y).count() as isize;

        (result - sensor_count - beacon_count) as usize
    }
}

impl ToString for Cave {
    fn to_string(&self) -> String {
        let mut x_max = isize::MIN;
        let mut y_max = isize::MIN;
        let mut x_min = isize::MAX;
        let mut y_min = isize::MAX;

        for (sensor, beacon) in self.map.map.clone() {
            if sensor.get_x().min(beacon.get_x()) < x_min {
                x_min = sensor.get_x().min(beacon.get_x());
            }
            if sensor.get_x().max(beacon.get_x()) > x_max {
                x_max = sensor.get_x().max(beacon.get_x());
            }

            if sensor.get_y().min(beacon.get_y()) < y_min {
                y_min = sensor.get_y().min(beacon.get_y());
            }
            if sensor.get_y().max(beacon.get_y()) > y_max {
                y_max = sensor.get_y().max(beacon.get_y());
            }
        }

        let mut tmp_map = vec![];
        for _ in 0..(y_max - y_min + 1) {
            tmp_map.push(vec!["."; (x_max - x_min + 1) as usize]);
        }

        for (sensor, beacon) in self.map.map.clone() {
            tmp_map.get_mut((sensor.get_y() - y_min) as usize).unwrap().remove((sensor.get_x() - x_min) as usize);
            tmp_map.get_mut((sensor.get_y() - y_min) as usize).unwrap().insert((sensor.get_x() - x_min) as usize, "S");
            tmp_map.get_mut((beacon.get_y() - y_min) as usize).unwrap().remove((beacon.get_x() - x_min) as usize);
            tmp_map.get_mut((beacon.get_y() - y_min) as usize).unwrap().insert((beacon.get_x() - x_min) as usize, "B");
        }

        for y in 0..tmp_map.len() {
            for x in 0..tmp_map.first().unwrap().len() {
                for (sensor, beacon) in self.map.map.clone() {
                    if tmp_map.get(y).unwrap().get(x).unwrap().eq(&".") {
                        let md = sensor.manhattan_distance(&beacon);
                        if sensor.manhattan_distance(&Point::new(x as isize + x_min, y as isize + y_min)) <= md {
                            tmp_map.get_mut((y) as usize).unwrap().remove((x) as usize);
                            tmp_map.get_mut((y) as usize).unwrap().insert((x) as usize, "#");
                        }
                    }
                }
            }
        }

        tmp_map
            .iter()
            .map(|column| column
                .iter()
                .fold("".to_string(), |acc, item| acc + item))
            .enumerate()
            .fold("".to_string(), |acc, (index, column)| format!("{}{:3} {}\n", acc, (index as isize) + y_min, column))
    }
}

#[cfg(test)]
mod tests {
    use crate::day_15::Cave;

    #[test]
    fn count_places_without_beacon() {
        let cave = Cave::new(include_str!("resources/test/day_15_cave_example.txt"));
        let places_without_beacon = cave.count_places_without_beacon_in_row(10);
        assert_eq!(places_without_beacon, 26);
    }

    #[test]
    fn find_unreachable_beacon_frequency() {
        let cave = Cave::new(include_str!("resources/test/day_15_cave_example.txt"));
        let (x, y) = cave.find_unreachable_beacon(0..=20);
        assert_eq!(x*4_000_000 + y, 56000011);
    }
}

