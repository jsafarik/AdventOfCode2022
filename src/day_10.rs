use std::str::FromStr;

#[allow(dead_code)]
pub fn main() {
    let input = include_str!("resources/day_10_cpu_instructions.txt");

    let mut cpu = CPU::new();
    for line in input.lines() {
        cpu.parse(line);
    }
    println!("The sum of significant cycles is {}", cpu.get_memory().iter().fold(0, |acc, x| acc + x));
}

struct CPU {
    cycle: u16,
    register_x: isize,
    memory: Vec<isize>,
}

impl CPU {
    fn new() -> CPU {
        CPU {
            cycle: 1,
            register_x: 1,
            memory: vec![],
        }
    }

    fn parse(&mut self, line: &str) {
        if line.contains("noop") {
            self.tick(0);
        } else {
            self.tick(0);
            let value = line.split_whitespace().collect::<Vec<&str>>().last().unwrap().clone();
            self.tick(isize::from_str(value).unwrap());
        }
    }

    fn tick(&mut self, value: isize) {
        if vec![20, 60, 100, 140, 180, 220].contains(&self.cycle) {
            self.memory.push(self.register_x * (self.cycle as isize));
        }

        if ((self.cycle as isize - 1) % 40) >= (self.register_x - 1) && ((self.cycle as isize - 1) % 40) <= (self.register_x + 1) {
            print!("#");
        } else {
            print!(".");
        }

        if (self.cycle % 40) == 0 {
            println!();
        }

        self.cycle += 1;
        self.register_x += value;
    }

    fn get_memory(&self) -> Vec<isize> {
        self.memory.clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::day_10::CPU;

    #[test]
    fn six_important_cycles_strength() {
        let input = include_str!("resources/test/day_10_cpu_instructions_example.txt");

        let mut cpu = CPU::new();
        for line in input.lines() {
            cpu.parse(line);
        }
        let correct_strengths = vec![420, 1140, 1800, 2940, 2880, 3960];
        assert_eq!(cpu.get_memory(), correct_strengths);
    }
}
