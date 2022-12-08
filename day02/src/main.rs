use std::fs::File;
use std::io::{self, BufRead};

#[derive(Clone, Copy)]
enum Outcome {
    Lost = 0,
    Draw = 3,
    Win = 6,
}

impl From<&str> for Outcome {
    fn from(outcome_char: &str) -> Self {
        match outcome_char {
            "X" => Outcome::Lost,
            "Y" => Outcome::Draw,
            "Z" => Outcome::Win,
            _ => panic!("Could not convert to Outcome: {}", outcome_char),
        }
    }
}

#[derive(Clone, Copy)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl From<&str> for Move {
    fn from(move_char: &str) -> Self {
        match move_char {
            "A" => Move::Rock,
            "B" => Move::Paper,
            "C" => Move::Scissors,
            _ => panic!("Could not convert to Move: {}", move_char),
        }
    }
}

fn get_move_for_outcome(opponent: Move, outcome: Outcome) -> Move {
    use crate::Move::*;
    use crate::Outcome::*;
    match (opponent, outcome) {
        (Rock, Win) => Paper,
        (Rock, Draw) => Rock,
        (Rock, Lost) => Scissors,
        (Paper, Win) => Scissors,
        (Paper, Draw) => Paper,
        (Paper, Lost) => Rock,
        (Scissors, Win) => Rock,
        (Scissors, Draw) => Scissors,
        (Scissors, Lost) => Paper,
    }
}

fn score(ours: Move, opponent: Move) -> i32 {
    use crate::Move::*;
    use crate::Outcome::*;

    let outcome = match (ours, opponent) {
        (Rock, Paper) => Lost,
        (Rock, Scissors) => Win,
        (Paper, Rock) => Win,
        (Paper, Scissors) => Lost,
        (Scissors, Rock) => Lost,
        (Scissors, Paper) => Win,
        (_, _) => Draw,
    };

    ours as i32 + outcome as i32
}

fn score_line(line: String) -> i32 {
    let mut split = line.split_whitespace();
    let opponent: Move = split.next().expect("Could not read opponent move").into();
    let outcome: Outcome = split
        .next()
        .expect("Could not read the expected outcome")
        .into();
    let ours: Move = get_move_for_outcome(opponent, outcome);
    score(ours, opponent)
}

fn main() {
    let file = File::open("./input.txt").expect("Unable to open file");
    let mut points = 0;
    for raw_line in io::BufReader::new(file).lines() {
        if let Ok(line) = raw_line {
            points += score_line(line)
        }
    }
    println!("Points: {}", points);
}
