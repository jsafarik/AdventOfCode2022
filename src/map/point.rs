use std::cmp::Ordering;

#[derive(Eq, Hash, PartialEq, Clone)]
pub struct Point {
    coordinates: (isize, isize),
}

impl Point {
    pub fn new(x: isize, y: isize) -> Point {
        Point {
            coordinates: (x, y)
        }
    }

    pub fn get_x(&self) -> isize {
        self.coordinates.0
    }

    pub fn get_y(&self) -> isize {
        self.coordinates.1
    }

    pub fn manhattan_distance(&self, other: &Self) -> usize {
        ((self.get_x() - other.get_x()).abs() + (self.get_y() - other.get_y()).abs()) as usize
    }
}

impl ToString for Point {
    fn to_string(&self) -> String {
        let (x, y) = self.coordinates;
        format!("{}:{}", x.to_string(), y.to_string())
    }
}

impl PartialOrd<Self> for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let ord = self.get_y().cmp(&other.get_y());
        return Some(if ord == Ordering::Equal { self.get_x().cmp(&other.get_x()) } else { ord });
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}
