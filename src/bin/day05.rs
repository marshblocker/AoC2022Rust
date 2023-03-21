use std::{fs};
use regex::Regex;

const INPUT_PATH: &str = "C:\\Users\\gabri\\OneDrive\\Documents\\Programming\\Personal\\AoC2022Rust\\input\\day05.txt";

fn read_input(path: &str) -> String {
    let content: String = fs::read_to_string(path).unwrap();

    content
}

fn main() {
    let input = read_input(INPUT_PATH);

    let mut stacks: [Vec<char>; 9] = [
        vec!['T', 'D', 'W', 'Z', 'V', 'P'],
        vec!['L', 'S', 'W', 'V', 'F', 'J', 'D'],
        vec!['Z', 'M', 'L', 'S', 'V', 'T', 'B', 'H'],
        vec!['R', 'S', 'J'],
        vec!['C', 'Z', 'B', 'G', 'F', 'M', 'L', 'W'],
        vec!['Q', 'W', 'V', 'H', 'Z', 'R', 'G', 'B'],
        vec!['V', 'J', 'P', 'C', 'B', 'D', 'N'],
        vec!['P', 'T', 'B', 'Q'],
        vec!['H', 'G', 'Z', 'R', 'C']
    ];

    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();

    // Part 1

    for line in input.lines() {
        let cap = re
            .captures_iter(line)
            .next()
            .unwrap();
        
        let amount = (&cap)[1]
            .parse::<i32>()
            .unwrap();
        let from = (&cap)[2]
            .parse::<usize>()
            .unwrap();
        let to = (&cap)[3]
            .parse::<usize>()
            .unwrap();

        for _ in 0..amount {
            let data = stacks[from-1].pop();
            
            if let Some(d) = data {
                stacks[to-1].push(d);
            } else {
                break;
            }
        }
    }

    let last_crates = stacks
        .iter()
        .map(|c| c.last().unwrap_or(&' '))
        .collect::<String>();

    println!("Part 1: {}", last_crates);

    // Part 2
    stacks = [
        vec!['T', 'D', 'W', 'Z', 'V', 'P'],
        vec!['L', 'S', 'W', 'V', 'F', 'J', 'D'],
        vec!['Z', 'M', 'L', 'S', 'V', 'T', 'B', 'H'],
        vec!['R', 'S', 'J'],
        vec!['C', 'Z', 'B', 'G', 'F', 'M', 'L', 'W'],
        vec!['Q', 'W', 'V', 'H', 'Z', 'R', 'G', 'B'],
        vec!['V', 'J', 'P', 'C', 'B', 'D', 'N'],
        vec!['P', 'T', 'B', 'Q'],
        vec!['H', 'G', 'Z', 'R', 'C']
    ];

    for line in input.lines() {
        let cap = re
            .captures_iter(line)
            .next()
            .unwrap();
        
        let amount = (&cap)[1]
            .parse::<i32>()
            .unwrap();
        let from = (&cap)[2]
            .parse::<usize>()
            .unwrap();
        let to = (&cap)[3]
            .parse::<usize>()
            .unwrap();

        let mut temp_stak: Vec<char> = Vec::new();

        for _ in 0..amount {
            let data = stacks[from-1].pop();
            
            if let Some(d) = data {
                temp_stak.push(d);
            } else {
                break;
            }
        }

        for _ in 0..amount {
            let data = temp_stak.pop();
            
            if let Some(d) = data {
                stacks[to-1].push(d);
            } else {
                break;
            }
        }
    }

    let last_crates = stacks
        .iter()
        .map(|c| c.last().unwrap_or(&' '))
        .collect::<String>();

    println!("Part 2: {}", last_crates);
}