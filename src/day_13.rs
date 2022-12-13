use std::cmp::Ordering;

pub fn main() {
    let input = include_str!("resources/day_13_packets.txt");
    let size = input
        .split("\n\n")
        .map(|pair| pair.split_once("\n").unwrap())
        .enumerate()
        .map(|(index, (a, b))| { (index + 1, Packet::parse(a.trim()).cmp(&Packet::parse(b.trim()))) })
        .fold(0, |acc, (index, status)| acc + if status == Status::Ok { index } else { 0 });
    println!("Sum of OK indices is {}", size);

    let divider_1 = "[[2]]";
    let divider_2 = "[[6]]";
    let mut list = (input.to_string() + "\n" + divider_1 + "\n" + divider_2)
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| Packet::parse(line))
        .collect::<Vec<Packet>>();

    list.sort_by(|a, b| if a.cmp(b) == Status::Ok { Ordering::Less } else { Ordering::Greater });
    let divider_1_pos = list.iter().position(|packet| *packet == Packet::parse(divider_1)).unwrap() + 1;
    let divider_2_pos = list.iter().position(|packet| *packet == Packet::parse(divider_2)).unwrap() + 1;
    println!("Indices of divisors multiplied is equal to {}", divider_1_pos*divider_2_pos);
}

impl PartialEq<Self> for Packet {
    fn eq(&self, other: &Self) -> bool {
        return if self.is_number() && other.is_number() {
            self.number.unwrap() == other.number.unwrap()
        } else if self.is_list() && other.is_list() {
            self.list.as_ref().unwrap() == other.list.as_ref().unwrap()
        } else {
            false
        }
    }
}

enum Status {
    Ok,
    Continue,
    Wrong,
}

impl PartialEq for Status {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}

impl ToString for Status {
    fn to_string(&self) -> String {
        match self {
            Status::Ok => "Ok".to_string(),
            Status::Wrong => "Wrong".to_string(),
            Status::Continue => "Continue".to_string(),
        }
    }
}

#[derive(Clone)]
struct Packet {
    number: Option<usize>,
    list: Option<Vec<Packet>>,
}

impl Packet {
    fn from_number(number: usize) -> Packet {
        Packet { number: Some(number), list: None }
    }

    fn from_list(list: Vec<Packet>) -> Packet {
        Packet { number: None, list: Some(list) }
    }

    fn parse(input: &str) -> Packet {
        let mut result: Vec<Packet> = vec![];

        let mut strip = input.strip_prefix("[").unwrap().strip_suffix("]").unwrap();
        while !strip.is_empty() {
            let mut first_part = "";
            strip = strip.strip_prefix(",").or(Some(strip)).unwrap();
            if strip.starts_with("[") {
                let mut num = 0;
                for (i, c) in strip.chars().enumerate() {
                    num += if c == '[' { 1 } else if c == ']' { -1 } else { 0 };
                    if num == 0 {
                        (first_part, strip) = strip.split_at(i + 1);
                        break;
                    }
                }
            } else {
                (first_part, strip) = strip.split_once(",").or(Some((strip, ""))).unwrap()
            }

            if first_part.starts_with("[") {
                result.push(Packet::parse(first_part));
            } else {
                result.push(Packet::from_number(first_part.parse().unwrap()));
            }
        }

        Packet::from_list(result)
    }

    fn is_number(&self) -> bool {
        self.number.is_some()
    }

    fn is_list(&self) -> bool {
        self.list.is_some()
    }

    #[allow(dead_code)]
    fn print(&self) {
        if self.is_number() {
            print!(" {} ", self.number.unwrap());
        } else if self.is_list() {
            print!("[");
            self.list.as_ref().unwrap().iter().for_each(|packet| packet.print());
            print!("]");
        } else {
            println!("Didnt recognize packet, empty!");
        }
    }

    fn cmp(&self, other: &Self) -> Status {
        if self.is_list() && other.is_list() {
            for (index, packet) in self.list.as_ref().unwrap().iter().enumerate() {
                if other.list.as_ref().unwrap().len() < index + 1 {
                    return Status::Wrong;
                }
                match packet.cmp(other.list.as_ref().unwrap().get(index).unwrap()) {
                    Status::Ok => { return Status::Ok; }
                    Status::Wrong => { return Status::Wrong; }
                    Status::Continue => continue
                }
            }

            if other.list.as_ref().unwrap().len() > self.list.as_ref().unwrap().len() {
                return Status::Ok;
            }

            return Status::Continue;
        } else if self.is_number() && other.is_number() {
            return if self.number.unwrap() < other.number.unwrap() {
                Status::Ok
            } else if self.number.unwrap() > other.number.unwrap() {
                Status::Wrong
            } else {
                Status::Continue
            };
        } else {
            return if self.is_number() {
                Packet::from_list(vec![self.clone()]).cmp(other)
            } else {
                self.cmp(&Packet::from_list(vec![other.clone()]))
            };
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;
    use crate::day_13::{Packet, Status};

    #[test]
    fn sum_of_ok_indices() {
        let size = include_str!("resources/test/day_13_packets_example.txt")
            .split("\n\n")
            .map(|pair| pair.split_once("\n").unwrap())
            .enumerate()
            .map(|(index, (a, b))| { (index + 1, Packet::parse(a.trim()).cmp(&Packet::parse(b.trim()))) })
            .fold(0, |acc, (index, status)| acc + if status == Status::Ok { index } else { 0 });
        assert_eq!(size, 13);
    }

    #[test]
    fn sort_and_divide() {
        let divider_1 = "[[2]]";
        let divider_2 = "[[6]]";
        let mut list = (include_str!("resources/test/day_13_packets_example.txt").to_string() + "\n" + divider_1 + "\n" + divider_2)
            .split("\n")
            .filter(|line| !line.is_empty())
            .map(|line| Packet::parse(line))
            .collect::<Vec<Packet>>();

        list.sort_by(|a, b| if a.cmp(b) == Status::Ok { Ordering::Less } else { Ordering::Greater });
        let divider_1_pos = list.iter().position(|packet| *packet == Packet::parse(divider_1)).unwrap() + 1;
        let divider_2_pos = list.iter().position(|packet| *packet == Packet::parse(divider_2)).unwrap() + 1;
        assert_eq!(divider_1_pos*divider_2_pos, 140);
    }
}
