use std::fs;

enum RockPaperScissorsMove {
    Rock,
    Paper,
    Scissors
}
use RockPaperScissorsMove::*;

impl RockPaperScissorsMove {
    fn from_char(c: char) -> RockPaperScissorsMove {
        match c {
            'A' => Rock,
            'B' => Paper,
            'C' => Scissors,
            'X' => Rock,
            'Y' => Paper,
            'Z' => Scissors,
            _ => panic!("unknown char for rock paper scissors")
        }
    }
}

enum RockPaperScissorsStrategy {
    Lose,
    Draw,
    Win
}
use RockPaperScissorsStrategy::*;

impl RockPaperScissorsStrategy {
    fn from_char(c: char) -> RockPaperScissorsStrategy {
        match c {
            'X' => Lose,
            'Y' => Draw,
            'Z' => Win,
            _ => panic!("unknown char for rock paper scissors strategy")
        }
    }
}

struct RockPaperScissorsGame { 
    their_move: RockPaperScissorsMove,
    my_move: RockPaperScissorsMove,
}

struct RockPaperScissorsGame2 { 
    their_move: RockPaperScissorsMove,
    strategy: RockPaperScissorsStrategy,
}

impl RockPaperScissorsGame {
    fn from_line(str: String) -> RockPaperScissorsGame {
        let move_chars : Vec<char> = str.split(' ').map(|word| word.chars().next().unwrap()).collect();
        RockPaperScissorsGame {
            their_move: RockPaperScissorsMove::from_char(move_chars[0]),
            my_move: RockPaperScissorsMove::from_char(move_chars[1]),
        }
    }


    fn score(self) -> i32 {
        match (self.their_move, self.my_move) {
            (Rock, Rock)         => 4, // draw (3) plus rock     (1)
            (Rock, Paper)        => 8, // win  (6) plus paper    (2)
            (Rock, Scissors)     => 3, // loss (0) plus scissors (3)
            (Paper, Rock)        => 1, // loss (0) plus rock     (1)
            (Paper, Paper)       => 5, // draw (3) plus paper    (2)
            (Paper, Scissors)    => 9, // win  (6) plus scissors (3)
            (Scissors, Rock)     => 7, // win  (6) plus rock     (1)
            (Scissors, Paper)    => 2, // loss (0) plus paper    (2)
            (Scissors, Scissors) => 6, // draw (3) plus scissors (3)
        }
    }

}
impl RockPaperScissorsGame2 {
    fn from_line(str: String) -> RockPaperScissorsGame2 {
        let move_chars : Vec<char> = str.split(' ').map(|word| word.chars().next().unwrap()).collect();
        RockPaperScissorsGame2 {
            their_move: RockPaperScissorsMove::from_char(move_chars[0]),
            strategy: RockPaperScissorsStrategy::from_char(move_chars[1]),
        }
    }

    fn score(self) -> i32 {
        match (self.their_move, self.strategy) {
            (Rock, Lose)     => 3, // lose (0) plus scissors (3)
            (Rock, Draw)     => 4, // draw (3) plus rock     (1)
            (Rock, Win)      => 8, // win  (6) plus paper    (2)
            (Paper, Lose)    => 1, // lose (0) plus rock     (1)
            (Paper, Draw)    => 5, // draw (3) plus paper    (2)
            (Paper, Win)     => 9, // win  (6) plus scissors (3)
            (Scissors, Lose) => 2, // lose (0) plus paper    (2)
            (Scissors, Draw) => 6, // draw (3) plus scissors (3)
            (Scissors, Win)  => 7, // win  (6) plus rock     (1)
        }
    }
}

fn main() {
    let input = fs::read_to_string("input").expect("Reading input file error");
    let input = input.lines();

    let sum_of_scores : i32 = input.clone()
        .map(|line| RockPaperScissorsGame::from_line(line.to_string()))
        .map(|game| game.score()).sum();

    println!("Sum of scores is {}", sum_of_scores);

    let sum_of_scores2: i32 = input
        .map(|line| RockPaperScissorsGame2::from_line(line.to_string()))
        .map(|game| game.score()).sum();

    println!("Sum of scores 2 is {}", sum_of_scores2);
}
