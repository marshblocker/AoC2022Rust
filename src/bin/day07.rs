use std::{fs};

const INPUT_PATH: &str = "C:\\Users\\gabri\\OneDrive\\Documents\\Programming\\Personal\\AoC2022Rust\\input\\day07.txt";

fn read_input(path: &str) -> String {
    let content: String = fs::read_to_string(path).unwrap();

    content
}

fn main() {
    let input = read_input(INPUT_PATH);

    // Part 1
    let mut total_weights = 0;
    let mut weights_stak: Vec<i32> = Vec::new();
    for line in input.lines() {
        match line.split(" ").collect::<Vec<_>>()[..] {
            ["$", "cd", ".."] => {
                let weight = weights_stak.pop().unwrap();
                if weight <= 100000 { total_weights += weight; }
                let parent_weight = weights_stak.pop().unwrap();
                weights_stak.push(parent_weight + weight);
            },
            ["$", "cd", _] => weights_stak.push(0),
            ["$", "ls"] => (),
            ["dir", _] => (),
            [w, _] => {
                let weight = weights_stak.pop().unwrap();
                weights_stak.push(weight + w.parse::<i32>().unwrap());
            },
            [..] => ()
        }
    }

    while weights_stak.len() > 0 {
        let weight = weights_stak.pop().unwrap();
        if weight <= 100000 { total_weights += weight; }
        if let Some(mut parent_weight) = weights_stak.pop() {
            parent_weight += weight;
            weights_stak.push(parent_weight);
        }
    }

    println!("Part 1: {}", total_weights);

    // Part 2
    let mut weights_stak: Vec<i32> = Vec::new();
    for line in input.lines() {
        match line.split(" ").collect::<Vec<_>>()[..] {
            ["$", "cd", ".."] => {
                let weight = weights_stak.pop().unwrap();
                if weight <= 100000 { total_weights += weight; }
                let parent_weight = weights_stak.pop().unwrap();
                weights_stak.push(parent_weight + weight);
            },
            ["$", "cd", _] => weights_stak.push(0),
            ["$", "ls"] => (),
            ["dir", _] => (),
            [w, _] => {
                let weight = weights_stak.pop().unwrap();
                weights_stak.push(weight + w.parse::<i32>().unwrap());
            },
            [..] => ()
        }
    }

    while weights_stak.len() > 1 {
        let weight = weights_stak.pop().unwrap();
        if weight <= 100000 { total_weights += weight; }
        if let Some(mut parent_weight) = weights_stak.pop() {
            parent_weight += weight;
            weights_stak.push(parent_weight);
        }
    }

    let total_weight = weights_stak.pop().unwrap();
    let need = 30000000 - (70000000 - total_weight);

    let mut candidate_weight = 100000000;
    let mut weights_stak: Vec<i32> = Vec::new();
    for line in input.lines() {
        match line.split(" ").collect::<Vec<_>>()[..] {
            ["$", "cd", ".."] => {
                let weight = weights_stak.pop().unwrap();
                if weight >= need && weight < candidate_weight { candidate_weight = weight; }
                let parent_weight = weights_stak.pop().unwrap();
                weights_stak.push(parent_weight + weight);
            },
            ["$", "cd", _] => weights_stak.push(0),
            ["$", "ls"] => (),
            ["dir", _] => (),
            [w, _] => {
                let weight = weights_stak.pop().unwrap();
                weights_stak.push(weight + w.parse::<i32>().unwrap());
            },
            [..] => ()
        }
    }

    while weights_stak.len() > 1 {
        let weight = weights_stak.pop().unwrap();
        if weight >= need && weight < candidate_weight { candidate_weight = weight; }
        if let Some(mut parent_weight) = weights_stak.pop() {
            parent_weight += weight;
            weights_stak.push(parent_weight);
        }
    }

    println!("Part 2: {}", candidate_weight);
}