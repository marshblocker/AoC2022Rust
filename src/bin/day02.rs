use std::{fs};

const INPUT_PATH: &str = "C:\\Users\\gabri\\OneDrive\\Documents\\Programming\\Personal\\advent-of-code-2022\\input\\day02.txt";

#[derive(PartialEq)]
enum Move {
    Rock,
    Paper,
    Scissors
}

impl Move {
    fn score(&self) -> i32 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3
        }
    }

    fn ret_move(chr: char) -> Result<Move, String> {
        match chr {
            'A'|'X' => Ok(Move::Rock),
            'B'|'Y' => Ok(Move::Paper),
            'C'|'Z' => Ok(Move::Scissors),
            other => Err(format!("Error: {} is not a valid character!", other))
        }
    }

    fn fight(&self, opp_move: Move) -> RoundOutcome {
        if *self == opp_move {
            return RoundOutcome::Draw;
        }

        if  (*self == Move::Paper && opp_move == Move::Rock) || 
            (*self == Move::Rock && opp_move == Move::Scissors) ||
            (*self == Move::Scissors && opp_move == Move::Paper) {
            return RoundOutcome::Win;
        }

        RoundOutcome::Lose
    }

    fn wins_to(&self) -> Move {
        match *self {
            Move::Rock => Move::Scissors,
            Move::Paper => Move::Rock,
            Move::Scissors => Move::Paper
        }
    }

    fn loses_to(&self) -> Move {
        match *self {
            Move::Rock => Move::Paper,
            Move::Paper => Move::Scissors,
            Move::Scissors => Move::Rock
        }
    }
}

enum RoundOutcome {
    Win,
    Lose,
    Draw
}

impl RoundOutcome {
    fn score(&self) -> i32 {
        match self {
            RoundOutcome::Win => 6,
            RoundOutcome::Lose => 0,
            RoundOutcome::Draw => 3
        }
    }

    fn ret_move(chr: char) -> Result<RoundOutcome, String> {
        match chr {
            'X' => Ok(RoundOutcome::Lose),
            'Y' => Ok(RoundOutcome::Draw),
            'Z' => Ok(RoundOutcome::Win),
            other => Err(format!("Error: {} is not a valid character!", other))
        }
    }
}

fn read_input(path: &str) -> String {
    let content: String = fs::read_to_string(path)
        .unwrap();

    content
}

fn main() {
    let input = read_input(INPUT_PATH);

    let lines: Vec<&str> = input.split("\r\n").collect();

    // Part 1
    let mut total_score = 0;
    for line in &lines {
        let opp_move = line.chars().nth(0).unwrap();
        let my_move = line.chars().nth(2).unwrap();

        let opp_move = Move::ret_move(opp_move).unwrap();
        let my_move = Move::ret_move(my_move).unwrap();

        let fight_res = my_move.fight(opp_move);
        total_score += fight_res.score();
        total_score += my_move.score();
    }

    println!("Part 1: {}", total_score);

    // Part 2
    total_score = 0;
    for line in &lines {
        let opp_move = line.chars().nth(0).unwrap();
        let fight_res = line.chars().nth(2).unwrap();

        let opp_move = Move::ret_move(opp_move).unwrap();
        let fight_res = RoundOutcome::ret_move(fight_res).unwrap();

        total_score += fight_res.score();
        total_score += match fight_res {
            RoundOutcome::Draw => opp_move.score(),
            RoundOutcome::Win => opp_move.loses_to().score(),
            RoundOutcome::Lose => opp_move.wins_to().score()
        }
    }

    println!("Part 2: {}", total_score);
}

