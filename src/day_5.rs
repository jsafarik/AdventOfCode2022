use std::collections::HashMap;

#[allow(dead_code)]
pub fn main() {
    let cranes_str = include_str!("resources/day_5_crates.txt");
    let mut to_rearrange = parse_initial_state(cranes_str);
    rearrange_crates(cranes_str, &mut to_rearrange, true);
    println!("Simple pop results: {}", get_top_items(to_rearrange));

    let mut to_rearrange = parse_initial_state(cranes_str);
    rearrange_crates(cranes_str, &mut to_rearrange, false);
    println!("Multi pop results: {}", get_top_items(to_rearrange));
}

#[derive(Debug)]
struct Stack<I> {
    stack: Vec<I>,
}

impl<I> Stack<I> {
    fn new() -> Stack<I> {
        Stack {
            stack: Vec::new()
        }
    }

    fn push(&mut self, value: I) {
        self.stack.push(value);
    }

    fn push_vector(&mut self, values: &mut Vec<I>) {
        self.stack.append(values);
    }

    fn pop(&mut self) -> I {
        if self.stack.len() >= 1 {
            self.stack.pop().unwrap()
        } else {
            panic!();
        }
    }

    /// Pop many elements from stack at once
    ///
    /// Instead of popping multiple elements one by one, this function doesn't reverse the order
    fn pop_many(&mut self, quantity: u32) -> Vec<I> {
        let mut rtrn = Vec::new();
        for item in self.stack.drain((self.stack.len() - usize::try_from(quantity).unwrap())..) {
            rtrn.push(item);
        }
        return rtrn;
    }

    fn get_last_item(&self) -> &I {
        self.stack.last().unwrap()
    }
}

impl<I: PartialEq> PartialEq for Stack<I> {
    fn eq(&self, other: &Self) -> bool {
        self.stack == other.stack
    }
}

fn parse_initial_state(rearrange_plan: &str) -> HashMap<u32, Stack<char>> {
    let (diagram, _) = rearrange_plan.split_once("\n\n").unwrap();

    let mut initial_state: HashMap<u32, Stack<char>> = HashMap::new();
    let mut column_positions: HashMap<u32, u32> = HashMap::new();

    for (idx, line) in diagram.lines().rev().enumerate() {
        if idx == 0 {
            line.chars().enumerate().for_each(|(pos, content)| {
                if !content.is_whitespace() {
                    let column_number = content.to_digit(10).unwrap();
                    initial_state.insert(column_number, Stack::new());
                    column_positions.insert(u32::try_from(pos).unwrap(), column_number);
                }
            });
        } else {
            line.chars().enumerate().for_each(|(pos, content)| {
                if content.is_alphabetic() {
                    initial_state
                        .get_mut(&column_positions[&u32::try_from(pos).unwrap()])
                        .unwrap()
                        .push(content);
                }
            })
        }
    }

    initial_state
}

fn rearrange_crates(rearrange_plan: &str, initial_plan: &mut HashMap<u32, Stack<char>>, simple_pop: bool) {
    let (_, movements) = rearrange_plan.split_once("\n\n").unwrap();

    movements.lines().for_each(|movement| {
        let values = movement
            .split_whitespace()
            .filter(|word| word.parse::<u32>().is_ok())
            .map(|num| num.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        if values.len() == 3 {
            let quantity = values[0];
            let from = values[1];
            let to = values[2];
            if simple_pop {
                for _ in 0..quantity {
                    let item = initial_plan.get_mut(&from).unwrap().pop();
                    initial_plan.get_mut(&to).unwrap().push(item);
                }
            } else {
                let mut items = initial_plan.get_mut(&from).unwrap().pop_many(quantity);
                initial_plan.get_mut(&to).unwrap().push_vector(&mut items);
            }
        } else {
            panic!();
        }
    });
}

fn get_top_items(crates_plan: HashMap<u32, Stack<char>>) -> String {
    let mut result = String::new();
    let from = crates_plan.keys().min().unwrap().clone();
    let to = crates_plan.keys().max().unwrap().clone() + 1;
    for idx in from..to {
        result += crates_plan[&u32::try_from(idx).unwrap().clone()].get_last_item().to_string().as_str();
    }
    result
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::day_5::{get_top_items, parse_initial_state, rearrange_crates, Stack};

    #[test]
    fn stack() {
        let mut stack = Stack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);
        assert_eq!(stack.stack, [1, 2, 3]);
        stack.push_vector(&mut vec![4, 5, 6]);
        assert_eq!(stack.stack, [1, 2, 3, 4, 5, 6]);
        assert_eq!(stack.pop(), 6);
        assert_eq!(stack.stack, [1, 2, 3, 4, 5]);
        assert_eq!(stack.pop_many(2), [4, 5]);
        assert_eq!(stack.stack, [1, 2, 3]);
    }

    fn get_initial_state() -> HashMap<u32, Stack<char>> {
        let mut expected = HashMap::new();
        let mut stack: Stack<char> = Stack::new();
        stack.push('Z');
        stack.push('N');
        expected.insert(1, stack);
        stack = Stack::new();
        stack.push('M');
        stack.push('C');
        stack.push('D');
        expected.insert(2, stack);
        stack = Stack::new();
        stack.push('P');
        expected.insert(3, stack);
        expected
    }

    fn get_rearranged_state_simple_pop() -> HashMap<u32, Stack<char>> {
        let mut expected = HashMap::new();
        let mut stack: Stack<char> = Stack::new();
        stack.push('C');
        expected.insert(1, stack);
        stack = Stack::new();
        stack.push('M');
        expected.insert(2, stack);
        stack = Stack::new();
        stack.push('P');
        stack.push('D');
        stack.push('N');
        stack.push('Z');
        expected.insert(3, stack);
        expected
    }

    fn get_rearranged_state_multi_pop() -> HashMap<u32, Stack<char>> {
        let mut expected = HashMap::new();
        let mut stack: Stack<char> = Stack::new();
        stack.push('M');
        expected.insert(1, stack);
        stack = Stack::new();
        stack.push('C');
        expected.insert(2, stack);
        stack = Stack::new();
        stack.push('P');
        stack.push('Z');
        stack.push('N');
        stack.push('D');
        expected.insert(3, stack);
        expected
    }

    #[test]
    fn parse_example_initial_state() {
        let example = include_str!("resources/test/day_5_crates_example.txt");
        let parsed = parse_initial_state(example);
        let expected = get_initial_state();
        assert_eq!(parsed, expected);
    }

    #[test]
    fn rearrange_simple_pop() {
        let example = include_str!("resources/test/day_5_crates_example.txt");
        let mut to_rearrange = get_initial_state();
        rearrange_crates(example, &mut to_rearrange, true);
        let expected = get_rearranged_state_simple_pop();
        assert_eq!(to_rearrange, expected);
    }

    #[test]
    fn rearrange_multi_pop() {
        let example = include_str!("resources/test/day_5_crates_example.txt");
        let mut to_rearrange = get_initial_state();
        rearrange_crates(example, &mut to_rearrange, false);
        let expected = get_rearranged_state_multi_pop();
        assert_eq!(to_rearrange, expected);
    }

    #[test]
    fn verify_top_items() {
        let expected = get_top_items(get_rearranged_state_multi_pop());
        assert_eq!(expected.as_str(), "MCD");
    }
}
