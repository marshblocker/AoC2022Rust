use std::{fs, collections::HashSet};

const INPUT_PATH: &str = "C:\\Users\\gabri\\OneDrive\\Documents\\Programming\\Personal\\AoC2022Rust\\input\\day03.txt";

fn read_input(path: &str) -> String {
    let content: String = fs::read_to_string(path).unwrap();

    content
}

fn to_score(chr: char) -> i32 {
    if chr.is_uppercase() {
        return chr as i32 - 'A' as i32 + 27;
    }

    chr as i32 - 'a' as i32 + 1
}

fn main() {
    let input = read_input(INPUT_PATH);

    // Part 1
    let mut score = 0;
    for line in input.lines() {
        let (left, right) = line
            .split_at((line.len() / 2) as usize);

        let left = left
            .chars()
            .collect::<HashSet<char>>();

        let right = right
            .chars()
            .collect::<HashSet<char>>();

        let intersection = left
            .intersection(&right)
            .collect::<HashSet<_>>();

        assert!(intersection.len() == 1);

        for chr in intersection {
            score += to_score(*chr);
        }
    }

    println!("Part 1: {}", score);

    // Part 2
    score = 0;

    let sacks = input
        .lines()
        .collect::<Vec<_>>();

    let sacks_iter = sacks
        .chunks_exact(3);
    
    for sack in sacks_iter {
        let first_sack = sack[0];
        let second_sack = sack[1];
        let third_sack = sack[2];

        let mut hset = first_sack
            .chars()
            .collect::<HashSet<_>>();

        let mut hset2 = second_sack
            .chars()
            .collect::<HashSet<_>>();

        hset.retain(|x| hset2.contains(x));

        hset2 = third_sack.chars().collect();

        hset.retain(|x| hset2.contains(x));

        let badge = hset.iter().next().unwrap();
        score += to_score(*badge);
    }

    println!("Part 2: {}", score);
}