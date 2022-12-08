pub fn main() {
    let communication = include_str!("resources/day_6_packet_transfer.txt");
    println!("Prefix is {} long", find_communication_prefix(communication, 4));
    println!("First communication after {} letters", find_communication_prefix(communication, 14))
}

fn find_communication_prefix(communication: &str, len: usize) -> u32 {
    let mut tmp_marker = vec![];
    for (idx, character) in communication.chars().enumerate() {
        tmp_marker.push(character);

        if tmp_marker.len() > len {
            tmp_marker.remove(0);
        }

        if tmp_marker.len() == len && !contains_duplicate(&tmp_marker) {
            return u32::try_from(idx).unwrap() + 1;
        }
    }

    panic!()
}

fn contains_duplicate(list: &Vec<char>) -> bool {
    let mut tmp = vec![];
    for character in list {
        if tmp.contains(character) {
            return true;
        }
        tmp.push(*character);
    }
    return false;
}

#[cfg(test)]
mod tests {
    use crate::day_6::find_communication_prefix;

    #[test]
    fn multiple_communications_find_prefix() {
        let multiple_communications = include_str!("resources/test/day_6_packet_transfer_multiline.txt").lines().collect::<Vec<&str>>();
        let correct_results: Vec<u32> = vec![7, 5, 6, 10, 11];
        let results = multiple_communications.iter().map(|comm| find_communication_prefix(comm, 4)).collect::<Vec<u32>>();
        assert_eq!(results, correct_results);
    }

    #[test]
    fn multiple_communications_find_communication() {
        let multiple_communications = include_str!("resources/test/day_6_packet_transfer_multiline.txt").lines().collect::<Vec<&str>>();
        let correct_results: Vec<u32> = vec![19, 23, 23, 29, 26];
        let results = multiple_communications.iter().map(|comm| find_communication_prefix(comm, 14)).collect::<Vec<u32>>();
        assert_eq!(results, correct_results);
    }
}

