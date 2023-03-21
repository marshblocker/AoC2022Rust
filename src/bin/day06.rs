use std::{fs, collections::VecDeque};

const INPUT_PATH: &str = "C:\\Users\\gabri\\OneDrive\\Documents\\Programming\\Personal\\AoC2022Rust\\input\\day06.txt";

fn read_input(path: &str) -> String {
    let content: String = fs::read_to_string(path).unwrap();

    content
}

fn main() {
    // Part 1
    let input = read_input(INPUT_PATH);
    let mut input = input.chars();

    let mut marker: VecDeque<char> = VecDeque::new();
    let mut i = 0;
    while marker.len() < 4 {
        let chr = input.next().unwrap();
        i += 1;

        while marker.contains(&chr) {
            marker.pop_front().unwrap();
        }
        
        marker.push_back(chr);
    }

    println!("Part 1: {}", i);

    // Part 2
    let input = read_input(INPUT_PATH);
    let mut input = input.chars();

    let mut marker: VecDeque<char> = VecDeque::new();
    let mut i = 0;
    while marker.len() < 14 {
        let chr = input.next().unwrap();
        i += 1;

        while marker.contains(&chr) {
            marker.pop_front().unwrap();
        }
        
        marker.push_back(chr);
    }

    println!("Part 2: {}", i);
}