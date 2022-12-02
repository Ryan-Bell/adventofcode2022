use std::fs::File;
use std::io::{self, BufRead};

#[derive(Clone, Copy)]
enum Outcome {
    Lost = 0,
    Draw = 3,
    Win = 6,
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
            "A" | "X" => Move::Rock,
            "B" | "Y" => Move::Paper,
            "C" | "Z" => Move::Scissors,
            _ => panic!("Could not convert to Move: {}", move_char),
        }
    }
}

fn score(ours: Move, opponent: Move) -> i32 {
    use crate::Move::*;
    use crate::Outcome::*;

    let outcome = match (ours, opponent) {
        (Rock, Rock) => Draw,
        (Rock, Paper) => Lost,
        (Rock, Scissors) => Win,
        (Paper, Rock) => Win,
        (Paper, Paper) => Draw,
        (Paper, Scissors) => Lost,
        (Scissors, Rock) => Lost,
        (Scissors, Paper) => Win,
        (Scissors, Scissors) => Draw,
    };

    ours as i32 + outcome as i32
}

fn score_line(line: String) -> i32 {
    let mut split = line.split_whitespace();
    let opponent: Move = split.next().expect("Could not read opponent move").into();
    let ours: Move = split.next().expect("Could not read our move").into();
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
