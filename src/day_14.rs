use std::collections::HashSet;

pub fn main() {
    let input = include_str!("resources/day_14.txt");
    let mut cave = Cave::new(input);

    let mut counter_touched_floor = 0;
    let mut counter_sand_reached_entrypoint = 0;
    let mut increment = 1;

    loop {
        let (r, d) = cave.tick();
        counter_touched_floor += increment;
        counter_sand_reached_entrypoint += 1;
        if d == cave.lowest_point + 1 {
            increment = 0;
        }
        if r == 500 && d == 0 {
            break;
        }
    }
    println!("{} units of sand before it starting free falling", counter_touched_floor - 1);
    println!("{} units of sand to start blocking sand entrypoint", counter_sand_reached_entrypoint);
}


struct Cave {
    obstacles: HashSet<(usize, usize)>,
    sand_entrypoint: (usize, usize),
    lowest_point: usize,
}

impl Cave {
    fn new(input: &str) -> Cave {
        let mut obstacles = HashSet::new();
        for line in input.lines() {
            let points = line.split(" -> ").collect::<Vec<&str>>();
            for point_index in 0..(points.len() - 1) {
                let (r1, d1) = points.get(point_index).unwrap().split_once(",").map(|(r, d)| (r.parse::<usize>().unwrap(), d.parse::<usize>().unwrap())).unwrap();
                let (r2, d2) = points.get(point_index + 1).unwrap().split_once(",").map(|(r, d)| (r.parse::<usize>().unwrap(), d.parse::<usize>().unwrap())).unwrap();
                for r in r1.min(r2)..=r1.max(r2) {
                    for d in d1.min(d2)..=d1.max(d2) {
                        obstacles.insert((r.clone(), d.clone()));
                    }
                }
            }
        }

        let mut cave = Cave {
            obstacles,
            sand_entrypoint: (500, 0),
            lowest_point: 0,
        };

        cave.cache_lowest_rock_point();

        cave
    }

    fn is_obstacle(&self, x: usize, y: usize) -> bool {
        self.obstacles.contains(&(x, y))
    }

    fn cache_lowest_rock_point(&mut self) {
        let mut o = self.obstacles.clone().into_iter().collect::<Vec<(usize, usize)>>();
        o.sort_by(|(_, a), (_, b)| b.cmp(a));

        self.lowest_point = o.first().unwrap().1
    }

    fn get_floor_level(&self) -> usize {
        self.lowest_point + 1
    }

    fn tick(&mut self) -> (usize, usize) {
        let (mut r, mut d) = self.sand_entrypoint.clone();
        loop {
            if self.get_floor_level() <= d {
                d = self.get_floor_level();
                break;
            } else if !self.is_obstacle(r, d + 1) {
                d += 1;
            } else if !self.is_obstacle(r - 1, d + 1) {
                r -= 1;
                d += 1;
            } else if !self.is_obstacle(r + 1, d + 1) {
                r += 1;
                d += 1;
            } else {
                break;
            }
        }
        self.obstacles.insert((r.clone(), d.clone()));
        (r, d)
    }
}

#[cfg(test)]
mod tests {
    use crate::day_14::Cave;

    #[test]
    fn units_before_free_falling() {
        let mut cave = Cave::new(include_str!("resources/test/day_14_example.txt"));
        let mut counter = 0;
        loop {
            let (_, d) = cave.tick();
            counter += 1;
            if d == cave.lowest_point + 1 {
                break;
            }
        }
        assert_eq!(counter - 1, 24);
    }

    #[test]
    fn units_to_reach_entrypoint() {
        let mut cave = Cave::new(include_str!("resources/test/day_14_example.txt"));
        let mut counter = 0;
        loop {
            let (r, d) = cave.tick();
            counter += 1;
            if r == 500 && d == 0 {
                break;
            }
        }
        assert_eq!(counter, 93);
    }
}
