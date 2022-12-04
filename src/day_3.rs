#[allow(dead_code)]
pub fn main() {
    let rucksacks_str = include_str!("resources/day_3_rucksacks.txt");
    let (misplaced_sum, badges_sum) = calculate_misplaced_types_and_badges_sum(rucksacks_str);
    println!("Sum of misplaced types priorities: {}", misplaced_sum);
    println!("Sum of badges: {}", badges_sum);
}

fn calculate_misplaced_types_and_badges_sum(rucksacks_str: &str) -> (usize, usize) {
    //a..zA..Z in a list, index corresponds to priority-1
    let types = ('a'..='z').chain('A'..='Z').collect::<Vec<char>>();

    let mut misplaced_sum = 0;
    let mut badges_sum = 0;
    let mut group: Vec<&str> = vec![];

    for rucksack in rucksacks_str.lines() {
        let (comp_1, comp_2) = rucksack.split_at(rucksack.len() / 2);
        let duplicate = comp_1
            .chars()
            .find(|item_type| comp_2.contains(item_type.to_string().as_str()))
            .expect("No duplicate was found");
        misplaced_sum += types.iter().position(|&item_type| item_type == duplicate).unwrap() + 1;

        group.push(rucksack);
        if group.len() == 3 {
            group.sort_by(|rs_1, rs_2| rs_1.len().cmp(&rs_2.len()));
            let badge = group.first()
                .expect("Group didn't have first element")
                .chars()
                .find(|item_type| group[1].contains(item_type.to_string().as_str()) && group[2].contains(item_type.to_string().as_str()))
                .expect("No badge found!");
            badges_sum += types.iter().position(|&item_type| item_type == badge).unwrap() + 1;
            group = vec![];
        }
    }

    (misplaced_sum, badges_sum)
}

#[cfg(test)]
mod tests {
    use crate::day_3::{calculate_misplaced_types_and_badges_sum};

    #[test]
    fn misplaced_types_and_badges_sum() {
        let rucksacks_str = include_str!("resources/test/day_3_rucksacks_example.txt");
        let (misplaced_sum, badges_sum) = calculate_misplaced_types_and_badges_sum(rucksacks_str);
        assert_eq!(misplaced_sum, 157);
        assert_eq!(badges_sum, 70);
    }
}
