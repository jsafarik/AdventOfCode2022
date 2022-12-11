use std::str::FromStr;
use regex::{Captures, Regex};

pub fn main() {
    let monkeys = include_str!("resources/day_11_monkeys.txt");
    let mut game = KeepAway::new(monkeys, 3);
    game.print();
    for _ in 0..20 {
        game.round();
    }
    game.print();
    println!("Monkey business with moderated stress is {}", game.get_monkey_business());

    let mut game = KeepAway::new(monkeys, 1);
    game.print();
    for _ in 0..10000 {
        game.round();
    }
    game.print();
    println!("Monkey business is {}", game.get_monkey_business());
}

struct KeepAway<'a> {
    monkeys: Vec<Monkey<'a>>,
    common_divisor: usize,
}

impl<'a> KeepAway<'a> {
    fn new(input: &str, stress_moderation: usize) -> KeepAway {
        let monkeys = input.split("\n\n").map(|monkey| Monkey::parse(monkey, stress_moderation)).collect::<Vec<Monkey>>();
        let mut divisors = monkeys.iter()
            .map(|monkey| usize::from_str(monkey.captures.as_ref().unwrap().name("test").unwrap().as_str()).unwrap())
            .collect::<Vec<usize>>();
        divisors.sort();
        divisors.dedup();
        KeepAway {
            monkeys,
            common_divisor: divisors.iter().fold(1, |acc, div| acc * div),
        }
    }

    fn print(&self) {
        self.monkeys.iter().for_each(|monkey| monkey.print());
    }

    fn round(&mut self) {
        let monkeys = &mut self.monkeys;
        for monkey in 0..monkeys.len() {
            for _ in 0..monkeys.get(monkey).unwrap().inventory.len() {
                let (index, value) = monkeys.get_mut(monkey).unwrap().turn();
                monkeys.get_mut(index).unwrap().inventory.push(value);
            }
        }
        for monkey in 0..monkeys.len() {
            monkeys.get_mut(monkey).unwrap().update_items(self.common_divisor);
        }
    }

    fn get_monkey_business(&self) -> usize {
        let mut inspected_list = self.monkeys.iter().map(|monkey| monkey.inspected).collect::<Vec<usize>>();
        inspected_list.sort_by(|a, b| b.cmp(a));
        inspected_list.first().unwrap() * inspected_list.get(1).unwrap()
    }
}

struct Monkey<'a> {
    inventory: Vec<usize>,
    index: usize,
    captures: Option<Captures<'a>>,
    inspected: usize,
    stress_moderation: usize,
}

impl<'a> Monkey<'a> {
    fn new(index: usize, starting_items: Vec<usize>, captures: Option<Captures>, stress_moderation: usize) -> Monkey {
        Monkey {
            inventory: starting_items,
            index,
            captures,
            inspected: 0,
            stress_moderation,
        }
    }

    fn inspect(&mut self, item: usize) -> usize {
        self.inspected += 1;
        let op = self.captures.as_ref().unwrap().name("op").unwrap().as_str();
        let num = if op.contains("old") { item } else { usize::from_str(op.split_whitespace().collect::<Vec<&str>>().get(1).unwrap()).unwrap() };
        if op.contains("*") {
            (item * num) / self.stress_moderation
        } else {
            (item + num) / self.stress_moderation
        }
    }

    fn test(&self, item: usize) -> usize {
        let division = usize::from_str(self.captures.as_ref().unwrap().name("test").unwrap().as_str()).unwrap();
        if (item % division) == 0 {
            usize::from_str(self.captures.as_ref().unwrap().name("yes").unwrap().as_str()).unwrap()
        } else {
            usize::from_str(self.captures.as_ref().unwrap().name("no").unwrap().as_str()).unwrap()
        }
    }

    fn parse(input: &str, stress_moderation: usize) -> Monkey {
        let regex = Regex::new("\
            Monkey (?P<monkey_index>\\d):\\n\\s\\s\
            Starting items: (?P<items>[\\w+,\\s]*)\\n\\s\\s\
            Operation: new = old (?P<op>.*)\\n\\s\\s\
            Test: divisible by (?P<test>\\d+)\\n\\s\\s\\s\\s\
            If true: throw to monkey (?P<yes>\\d+)\\n\\s\\s\\s\\s\
            If false: throw to monkey (?P<no>\\d+)\
            ").unwrap();

        let captures = regex.captures(input).unwrap();
        Monkey::new(
            usize::from_str(captures.name("monkey_index").unwrap().as_str()).unwrap(),
            captures.name("items").unwrap().as_str().split(", ").map(|cap| usize::from_str(cap).unwrap()).collect::<Vec<usize>>(),
            Some(captures),
            stress_moderation,
        )
    }

    fn print(&self) {
        println!("{}:{} | {}", self.index, self.inventory.iter().fold("".to_string(), |acc, item| acc + " " + item.to_string().as_str()), self.inspected);
    }

    fn turn(&mut self) -> (usize, usize) {
        let mut item = self.inventory.first().unwrap().clone();
        self.inventory.remove(0);
        item = self.inspect(item);
        (self.test(item), item)
    }

    fn update_items(&mut self, divisor: usize) {
        let current_items = self.inventory.clone();
        let mut new_items = vec![];
        for item in current_items {
            new_items.push(item % divisor);
        }
        self.inventory = new_items;
    }
}

#[cfg(test)]
mod tests {
    use crate::day_11::KeepAway;

    #[test]
    fn count_monkey_business_moderated_stress() {
        let monkeys = include_str!("resources/test/day_11_monkeys_example.txt");
        let mut game = KeepAway::new(monkeys, 3);
        for _ in 0..20 {
            game.round();
        }
        assert_eq!(game.get_monkey_business(), 10605);
    }

    #[test]
    fn count_monkey_business() {
        let monkeys = include_str!("resources/test/day_11_monkeys_example.txt");
        let mut game = KeepAway::new(monkeys, 1);
        for _ in 0..10000 {
            game.round();
        }
        assert_eq!(game.get_monkey_business(), 2713310158);
    }
}

