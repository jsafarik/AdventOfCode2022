#[allow(dead_code)]
pub fn main() {
    let cleaning_plan_str = include_str!("resources/day_4_cleaning.txt");
    println!("There are {} fully contained plans", count_contained(cleaning_plan_str, true));
    println!("There are {} partially contained plans", count_contained(cleaning_plan_str, false));
}

fn count_contained(cleaning_plan_str: &str, fully_contained: bool) -> u32 {
    cleaning_plan_str
        .lines()
        .map(|pair| {
            let list = pair.split(",").collect::<Vec<&str>>();
            (list[0], list[1])
        })
        .map(|(first_elf, second_elf)| {
            let first_split = first_elf.split("-").map(|str| str.parse::<u32>().expect("Couldn't parse")).collect::<Vec<u32>>();
            let second_split = second_elf.split("-").map(|str| str.parse::<u32>().expect("Couldn't parse")).collect::<Vec<u32>>();
            ((first_split[0], first_split[1]), (second_split[0], second_split[1]))
        })
        .fold(0, |acc, pair_ranges| {
            if fully_contained {
                count_fully_contained(pair_ranges, acc)
            } else {
                count_partially_contained(pair_ranges, acc)
            }
        })
}

fn count_fully_contained(pair_ranges: ((u32, u32), (u32, u32)), acc: u32) -> u32 {
    let ((first_start, first_end), (second_start, second_end)) = pair_ranges;
    if (first_start.ge(&second_start) && first_end.le(&second_end)) || (second_start.ge(&first_start) && second_end.le(&first_end)) {
        acc + 1
    } else {
        acc
    }
}

fn count_partially_contained(pair_ranges: ((u32, u32), (u32, u32)), acc: u32) -> u32 {
    let ((first_start, first_end), (second_start, second_end)) = pair_ranges;
    if is_between(first_start, second_start, second_end) ||
        is_between(first_end, second_start, second_end) ||
        is_between(second_start, first_start, first_end) ||
        is_between(second_start, first_start, first_end) {
        acc + 1
    } else {
        acc
    }
}

fn is_between(num: u32, start: u32, end: u32) -> bool {
    if num.ge(&start) && num.le(&end) {
        return true;
    }
    false
}

#[cfg(test)]
mod tests {
    use crate::day_4::count_contained;

    #[test]
    fn fully_contained() {
        let cleaning_plan_str = include_str!("resources/test/day_4_cleaning_example.txt");
        assert_eq!(count_contained(cleaning_plan_str, true), 2);
    }

    #[test]
    fn partially_contained() {
        let cleaning_plan_str = include_str!("resources/test/day_4_cleaning_example.txt");
        assert_eq!(count_contained(cleaning_plan_str, false), 4);
    }
}
