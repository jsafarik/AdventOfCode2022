use std::str::FromStr;

pub fn main() {
    let input = include_str!("resources/day_9_ropes.txt");
    let mut grid = Grid::new(2);
    for (direction, cycles) in input.lines().map(|line| Direction::parse(line)) {
        for _ in 0..cycles {
            grid.move_head(&direction);
        }
    }
    println!("Tail has visited {} positions", grid.count_visited());


    grid = Grid::new(10);
    for (direction, cycles) in input.lines().map(|line| Direction::parse(line)) {
        for _ in 0..cycles {
            grid.move_head(&direction);
        }
    }
    println!("Tail has visited {} positions", grid.count_visited());
}

enum Direction {
    Right,
    Left,
    Up,
    Down,
}

impl Direction {
    fn parse(line: &str) -> (Direction, usize) {
        let parts = line.split_whitespace().collect::<Vec<&str>>();
        match parts[0].to_lowercase() {
            x if x.contains("r") => (Direction::Right, usize::from_str(parts[1]).unwrap()),
            x if x.contains("l") => (Direction::Left, usize::from_str(parts[1]).unwrap()),
            x if x.contains("u") => (Direction::Up, usize::from_str(parts[1]).unwrap()),
            x if x.contains("d") => (Direction::Down, usize::from_str(parts[1]).unwrap()),
            _ => panic!()
        }
    }
}

struct Grid {
    positions: Vec<Position>,
    rope: Vec<usize>,
}

impl Grid {
    fn new(knots: usize) -> Grid {
        Grid {
            positions: vec![Position { x: 0, y: 0, visited: true }],
            rope: vec![0; knots],
        }
    }

    fn move_head(&mut self, direction: &Direction) {
        let current_head = self.get_current_head_position();
        let index = match direction {
            Direction::Right => { self.get_position_index(current_head.x + 1, current_head.y) }
            Direction::Left => { self.get_position_index(current_head.x - 1, current_head.y) }
            Direction::Up => { self.get_position_index(current_head.x, current_head.y - 1) }
            Direction::Down => { self.get_position_index(current_head.x, current_head.y + 1) }
        };
        *self.rope.first_mut().unwrap() = index;
        for x in 0..(self.rope.len() - 1) {
            self.recalculate_knots(x, x + 1);
        }
    }

    fn recalculate_knots(&mut self, knot_index_1: usize, knot_index_2: usize) {
        let (head_x, head_y) = self.positions
            .get(*self.rope.get(knot_index_1).unwrap())
            .map(|pos| (pos.x, pos.y))
            .unwrap();
        let (tail_x, tail_y) = self.positions
            .get(*self.rope.get(knot_index_2).unwrap())
            .map(|pos| (pos.x, pos.y))
            .unwrap();
        let diff_x = head_x - tail_x;
        let diff_y = head_y - tail_y;
        if diff_x > 1 || diff_y > 1 || diff_x < -1 || diff_y < -1 {
            if diff_x.abs() >= 1 && diff_y.abs() >= 1 {
                *self.rope.get_mut(knot_index_2).unwrap() = self.get_position_index(tail_x + (diff_x / diff_x.abs()), tail_y + (diff_y / diff_y.abs()));
            } else {
                *self.rope.get_mut(knot_index_2).unwrap() = self.get_position_index(
                    if diff_x == 0 { tail_x } else { tail_x + (diff_x / diff_x.abs()) },
                    if diff_y == 0 { tail_y } else { tail_y + (diff_y / diff_y.abs()) },
                );
            }
            if knot_index_2 == (self.rope.len() - 1) {
                self.positions.get_mut(*self.rope.get(knot_index_2).unwrap()).unwrap().visited = true;
            }
        }
    }

    fn get_current_head_position(&self) -> &Position {
        self.positions.get(*self.rope.first().unwrap()).unwrap()
    }

    fn get_position_index(&mut self, x: i16, y: i16) -> usize {
        let mut index = self.positions.iter().position(|pos| pos.x == x && pos.y == y);
        if index.is_none() {
            self.positions.push(Position { x, y, visited: false });
            index = Some(self.positions.len() - 1);
        }
        index.unwrap()
    }

    fn count_visited(&self) -> usize {
        self.positions.iter().filter(|pos| pos.visited).count()
    }
}

#[derive(Copy, Clone)]
struct Position {
    x: i16,
    y: i16,
    visited: bool,
}

#[cfg(test)]
mod tests {
    use crate::day_9::{Direction, Grid};

    #[test]
    fn count_visited_positions_two_knots() {
        let input = include_str!("resources/test/day_9_ropes_example.txt");
        let mut grid = Grid::new(2);
        for (direction, cycles) in input.lines().map(|line| Direction::parse(line)) {
            for _ in 0..cycles {
                grid.move_head(&direction);
            }
        }
        assert_eq!(grid.count_visited(), 13);
    }

    #[test]
    fn count_visited_positions_ten_knots() {
        let input = include_str!("resources/test/day_9_ropes_example.txt");
        let mut grid = Grid::new(10);
        for (direction, cycles) in input.lines().map(|line| Direction::parse(line)) {
            for _ in 0..cycles {
                grid.move_head(&direction);
            }
        }
        assert_eq!(grid.count_visited(), 1);
    }
}

