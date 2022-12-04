#[allow(dead_code)]
pub fn main() {
    let game_str = include_str!("resources/day_2_rock_paper_scissors.txt");
    println!("Incorrect strategy: {}", calculate_score(game_str, false));
    println!("Correct strategy: {}", calculate_score(game_str, true));
}

fn calculate_score(game_str: &str, correct_strategy: bool) -> u32 {
    let mut score = 0;
    game_str
        .lines()
        .map(|line| RpsGame::get_round_moves(line, correct_strategy))
        .for_each(|(opponent, me)| score += me.play(&opponent) + me.get_shape_value());
    return score;
}

enum RpsGame {
    Rock,
    Paper,
    Scissors,
}

impl RpsGame {
    fn get_move(movement: &str) -> RpsGame {
        match movement {
            "A" | "X" => RpsGame::Rock,
            "B" | "Y" => RpsGame::Paper,
            "C" | "Z" => RpsGame::Scissors,
            _ => panic!()
        }
    }

    fn get_my_move_on_opponent(me: &str, opponent: &str) -> RpsGame {
        match me {
            "Y" => Self::get_move(opponent),
            "Z" => if matches!(opponent, "A") { RpsGame::Paper } else { if matches!(opponent, "B") { RpsGame::Scissors } else { RpsGame::Rock } },
            "X" => if matches!(opponent, "A") { RpsGame::Scissors } else { if matches!(opponent, "B") { RpsGame::Rock } else { RpsGame::Paper } },
            _ => panic!()
        }
    }

    fn get_round_moves(movements: &str, correct_strategy: bool) -> (RpsGame, RpsGame) {
        let (move_1, move_2) = movements.split_once(" ").expect("Couldn't split moves");
        (Self::get_move(move_1), if correct_strategy { Self::get_my_move_on_opponent(move_2, move_1) } else { Self::get_move(move_2) })
    }

    fn play(&self, opponent: &RpsGame) -> u32 {
        if self.get_shape_value() == opponent.get_shape_value() {
            return 3;
        }

        match self {
            RpsGame::Rock => if matches!(opponent, RpsGame::Scissors) { 6 } else { 0 },
            RpsGame::Paper => if matches!(opponent, RpsGame::Rock) { 6 } else { 0 },
            RpsGame::Scissors => if matches!(opponent, RpsGame::Paper) { 6 } else { 0 },
        }
    }

    fn get_shape_value(&self) -> u32 {
        match self {
            RpsGame::Rock => 1,
            RpsGame::Paper => 2,
            RpsGame::Scissors => 3
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::day_2::calculate_score;

    #[test]
    fn example_game_incorrect_strategy() {
        let game_str = include_str!("resources/test/day_2_rock_paper_scissors_example.txt");
        assert_eq!(calculate_score(game_str, false), 15)
    }

    #[test]
    fn example_game_correct_strategy() {
        let game_str = include_str!("resources/test/day_2_rock_paper_scissors_example.txt");
        assert_eq!(calculate_score(game_str, true), 12)
    }
}
