pub fn main() {
    let input = include_str!("resources/day_12_height_map.txt");
    let mut map = Map::new(input, 'S', true);
    map.update_all_directions();
    map.update_all_weights();
    println!("End weight is {}", map.get_weights('E').first().unwrap());

    let mut map = Map::new(input, 'E', false);
    map.update_all_directions();
    map.update_all_weights();
    println!("'a' closest to end has weight {}", map.get_weights('a').iter().min().unwrap());
}

#[derive(Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl ToString for Direction {
    fn to_string(&self) -> String {
        match self {
            Direction::Up => "^".to_string(),
            Direction::Down => "v".to_string(),
            Direction::Left => "<".to_string(),
            Direction::Right => ">".to_string(),
        }
    }
}

#[derive(Clone)]
struct Position {
    x: usize,
    y: usize,
    value: char,
    weight: usize,
    visited: bool,
    directions: Vec<Direction>,
}

struct Map {
    map: Vec<Position>,
    width: usize,
    start: usize,
    ascending: bool,
}

impl Map {
    fn new(input: &str, s: char, ascending: bool) -> Map {
        let mut map = vec![];
        let mut start: usize = 0;
        for (list_index, line) in input.lines().enumerate() {
            for (char_index, position) in line.chars().enumerate() {
                map.push(Position {
                    x: char_index,
                    y: list_index,
                    value: position,
                    weight: usize::MAX,
                    visited: false,
                    directions: vec![],
                });
                if position == s {
                    start = map.len() - 1;
                }
            }
        }
        Map { map, start, width: input.lines().last().unwrap().len(), ascending }
    }

    fn update_all_directions(&mut self) {
        for position in 0..self.map.len() {
            let mut directions = vec![];
            for direction in vec![Direction::Right, Direction::Left, Direction::Up, Direction::Down] {
                let me = self.map.get(position).unwrap();
                let neighbor = self.get_neighbor(me.x, me.y, direction);
                if neighbor.is_some() &&
                    ((self.ascending && Map::get_value(neighbor.unwrap().value) as usize <= Map::get_value(me.value) + 1) ||
                        (!self.ascending && Map::get_value(neighbor.unwrap().value) as usize >= Map::get_value(me.value) - 1)) {
                    directions.push(direction)
                }
            }
            self.map.get_mut(position).unwrap().directions = directions;
        }
    }

    fn update_all_weights(&mut self) {
        let mut todo: Vec<usize> = vec![self.start.clone()];
        self.map.get_mut(self.start).unwrap().weight = 0;
        while todo.len() != 0 {
            let item = todo.remove(0);
            self.map.get_mut(item).unwrap().visited = true;
            let me = self.map.get(item).unwrap().clone();
            for direction in me.directions {
                let width = self.width.clone();
                let neighbor = self.get_mut_neighbor(me.x, me.y, direction).unwrap();
                if !neighbor.visited && !todo.contains(&(neighbor.y * width + neighbor.x)) {
                    todo.push(neighbor.y * width + neighbor.x);
                }
                if neighbor.weight > me.weight + 1 {
                    neighbor.weight = me.weight + 1;
                }
            }
        }
    }

    fn get_value(value: char) -> usize {
        if value == 'S' {
            return 'a' as usize;
        } else if value == 'E' {
            return 'z' as usize;
        }

        value as usize
    }

    fn get_neighbor(&self, x: usize, y: usize, direction: Direction) -> Option<&Position> {
        let (neighbor_x, neighbor_y) = match direction {
            Direction::Left => (if x == 0 { usize::MAX } else { x - 1 }, y),
            Direction::Right => (x + 1, y),
            Direction::Up => (x, if y == 0 { usize::MAX } else { y - 1 }),
            Direction::Down => (x, y + 1),
        };
        self.get_position(neighbor_x, neighbor_y)
    }

    fn get_mut_neighbor(&mut self, x: usize, y: usize, direction: Direction) -> Option<&mut Position> {
        let (neighbor_x, neighbor_y) = match direction {
            Direction::Left => (if x == 0 { usize::MAX } else { x - 1 }, y),
            Direction::Right => (x + 1, y),
            Direction::Up => (x, if y == 0 { usize::MAX } else { y - 1 }),
            Direction::Down => (x, y + 1),
        };
        self.get_mut_position(neighbor_x, neighbor_y)
    }

    fn get_position(&self, x: usize, y: usize) -> Option<&Position> {
        if x >= self.width || y >= self.get_height() {
            return None;
        }

        Some(self.map.get(y * self.width + x).unwrap())
    }

    fn get_mut_position(&mut self, x: usize, y: usize) -> Option<&mut Position> {
        if x >= self.width || y >= self.get_height() {
            return None;
        }

        Some(self.map.get_mut(y * self.width + x).unwrap())
    }

    fn get_height(&self) -> usize {
        self.map.len() / self.width
    }

    fn get_weights(&self, value: char) -> Vec<usize> {
        self.map.iter().filter(|pos| pos.value == value).map(|pos| pos.weight).collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::day_12::Map;

    #[test]
    fn shortest_path_steps() {
        let input = include_str!("resources/test/day_12_height_map_example.txt");
        let mut map = Map::new(input, 'S', true);
        map.update_all_directions();
        map.update_all_weights();
        assert_eq!(*map.get_weights('E').first().unwrap(), 31);
    }

    #[test]
    fn lowest_point_to_end_shortest_path() {
        let input = include_str!("resources/test/day_12_height_map_example.txt");
        let mut map = Map::new(input, 'E', false);
        map.update_all_directions();
        map.update_all_weights();
        assert_eq!(*map.get_weights('a').iter().min().unwrap(), 29);
    }
}
