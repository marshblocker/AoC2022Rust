use std::{fs};

const INPUT_PATH: &str = "C:\\Users\\gabri\\OneDrive\\Documents\\Programming\\Personal\\advent-of-code-2022\\input\\day04.txt";

fn read_input(path: &str) -> String {
    let content: String = fs::read_to_string(path).unwrap();

    content
}

fn main() {
    let input = read_input(INPUT_PATH);

    // Part 1 & 2
    let mut contains = 0;
    let mut overlaps = 0;
    for line in input.lines() {
        let mut line = line.split(',');
        let a = line.next().unwrap();
        let b = line.next().unwrap();

        let mut a = a.split('-');
        let a_low  = a
            .next()
            .unwrap()
            .parse::<i32>()
            .unwrap();
        let a_high = a
            .next()
            .unwrap()
            .parse::<i32>()
            .unwrap();

        let mut b = b.split('-');
        let b_low  = b
            .next()
            .unwrap()
            .parse::<i32>()
            .unwrap();
        let b_high = b
            .next()
            .unwrap()
            .parse::<i32>()
            .unwrap();

        if (a_low <= b_low && a_high >= b_high)
        || (b_low <= a_low && b_high >= a_high)  {
            contains += 1;
        }

        // 1-3,3,5
        if (a_low <= b_low && a_high >= b_low)
        || (b_low <= a_low && b_high >= a_low) {
            overlaps += 1;
        }
    }

    println!("Part 1: {}", contains);
    println!("Part 2: {}", overlaps);
}