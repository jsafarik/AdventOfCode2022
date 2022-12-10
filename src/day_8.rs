#[allow(dead_code)]
pub fn main() {
    let forest = Forest::new(include_str!("resources/day_8_forest.txt"));

    forest.print();
    println!("Number of visible trees: {}", forest.count_all_visible());
    println!("The best scenic view has score of {}", forest.get_best_scenic_score());
}


struct Forest {
    width: u16,
    trees: Vec<u8>,
}

impl Forest {
    fn new(input: &str) -> Forest {
        Forest {
            width: input.lines().last().unwrap().len() as u16,
            trees: input.lines().map(|line| line.chars().map(|letter| letter.to_digit(10).unwrap() as u8)).flatten().collect(),
        }
    }

    fn right(&self, x: u16, y: u16) -> DirectionalIterator {
        self.create_directional_iterator(x, y, |x| x + 1, |y| y)
    }

    fn left(&self, x: u16, y: u16) -> DirectionalIterator {
        self.create_directional_iterator(x, y, |x| if x == 0 { u16::MAX } else { x - 1 }, |y| y)
    }

    fn up(&self, x: u16, y: u16) -> DirectionalIterator {
        self.create_directional_iterator(x, y, |x| x, |y| if y == 0 { u16::MAX } else { y - 1 })
    }

    fn down(&self, x: u16, y: u16) -> DirectionalIterator {
        self.create_directional_iterator(x, y, |x| x, |y| y + 1)
    }

    fn create_directional_iterator<F: Fn(u16) -> u16, G: Fn(u16) -> u16>(&self, x: u16, y: u16, x_closure: F, y_closure: G) -> DirectionalIterator {
        let mut vec: Vec<u8> = Vec::new();

        let mut index_x = x_closure(x);
        let mut index_y = y_closure(y);
        loop {
            let tree = self.get_tree(index_x, index_y);
            if tree.is_none() {
                break;
            }
            vec.push(tree.unwrap());
            index_x = x_closure(index_x);
            index_y = y_closure(index_y);
        }

        DirectionalIterator { list: vec }
    }

    fn get_height(&self) -> u16 {
        self.trees.len() as u16 / self.width
    }

    fn two_dim_to_one_dim(&self, x: u16, y: u16) -> Option<usize> {
        if x >= self.width || y >= self.get_height() {
            return None;
        }
        return Some((self.width * y + x) as usize);
    }

    fn one_dim_to_two_dim(&self, a: usize) -> (u16, u16) {
        ((a % self.width as usize) as u16, (a / self.width as usize) as u16)
    }

    fn get_tree(&self, x: u16, y: u16) -> Option<u8> {
        let a = self.two_dim_to_one_dim(x, y);
        self.trees.get(a.or(Some(usize::MAX)).unwrap()).cloned()
    }

    fn print(&self) {
        for (i, x) in self.trees.iter().enumerate() {
            if i % self.width as usize == 0 {
                print!("\n");
            }
            print!("{}", x);
        }
        print!("\n");
    }

    fn count_all_visible(&self) -> usize {
        let mut counter = 0;

        for x in 0..self.width {
            for y in 0..self.get_height() {
                if self.is_tree_visible(x, y) {
                    counter += 1;
                }
            }
        }

        counter
    }

    fn is_tree_visible(&self, x: u16, y: u16) -> bool {
        if self.is_tree_on_edge(x, y) {
            return true;
        }

        let tree_height = self.get_tree(x, y).unwrap();
        let predicate = |tree: &u8| *tree >= tree_height;

        self.right(x, y).find(&predicate).is_none() ||
            self.left(x, y).find(&predicate).is_none() ||
            self.up(x, y).find(&predicate).is_none() ||
            self.down(x, y).find(&predicate).is_none()
    }

    fn is_tree_on_edge(&self, x: u16, y: u16) -> bool {
        x == 0 || x == (self.width - 1) || y == 0 || y == (self.get_height() - 1)
    }

    fn get_scenic_score(&self, x: u16, y: u16) -> usize {
        let tree_height = self.get_tree(x, y).unwrap();
        let predicate = |(_, tree): &(usize, u8)| *tree >= tree_height;
        let add_one = |(index, tree)| (index + 1, tree);
        let (left, _) = self.left(x, y).enumerate().map(&add_one).find(&predicate).or(Some((x as usize, 0))).unwrap();
        let (right, _) = self.right(x, y).enumerate().map(&add_one).find(&predicate).or(Some(((self.width - 1 - x) as usize, 0))).unwrap();
        let (up, _) = self.up(x, y).enumerate().map(&add_one).find(&predicate).or(Some((y as usize, 0))).unwrap();
        let (down, _) = self.down(x, y).enumerate().map(&add_one).find(&predicate).or(Some(((self.get_height() - 1 - y) as usize, 0))).unwrap();
        left * right * up * down
    }

    fn get_best_scenic_score(&self) -> usize {
        let mut score = 0;
        for x in 0..self.width {
            for y in 0..self.get_height() {
                let s = self.get_scenic_score(x, y);
                if s > score {
                    score = s;
                }
            }
        }
        score
    }
}

struct DirectionalIterator {
    list: Vec<u8>,
}

impl Iterator for DirectionalIterator {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.list.len() > 0 {
            let first_item = self.list.first().unwrap().clone();
            self.list.remove(0);
            Some(first_item)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::day_8::Forest;

    #[test]
    fn count_visible_trees() {
        let input = include_str!("resources/test/day_8_forest_example.txt");
        let forest = Forest::new(input);
        assert_eq!(forest.count_all_visible(), 21);
    }

    #[test]
    fn get_best_scenic_score() {
        let input = include_str!("resources/test/day_8_forest_example.txt");
        let forest = Forest::new(input);
        assert_eq!(forest.get_best_scenic_score(), 8);
    }
}
