fn main() {
    let inventory_str = include_str!("resources/day_1_elves_inventory.txt");
    let (index, calories) = get_elf_with_most_food_in_calories(inventory_str);
    println!("Elf #{} has the most calories ({})", index, calories);
}

fn get_elf_with_most_food_in_calories(inventory_str: &str) -> (usize, u32) {
    inventory_str
        .split("\n\n")
        .map(|inv: &str| inv
            .lines()
            .map(|cal: &str| cal.parse::<u32>().expect("Couldn't parse calories input to u32"))
            .fold(0, |acc: u32, x: u32| acc + x)
        )
        .enumerate()
        .max_by(|(_, a), (_, b)| a.cmp(b))
        .expect("Couldn't find max value")
}

#[cfg(test)]
mod tests {
    use crate::get_elf_with_most_food_in_calories;

    #[test]
    fn example_inventory_max_calories() {
        let inventory_str = include_str!("resources/test/day_1_elves_inventory_example.txt");
        let (index, calories) = get_elf_with_most_food_in_calories(inventory_str);
        let results: [u32; 5] = [6000, 4000, 11000, 24000, 10000];
        for (idx, result) in results.iter().enumerate() {
            if 24000.eq(result) {
                assert_eq!(index, idx);
                assert_eq!(calories, result.clone());
                continue;
            }
            assert_ne!(index, idx);
            assert_ne!(calories, result.clone());
        }
    }
}
