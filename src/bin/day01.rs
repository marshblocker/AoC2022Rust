use std::{fs};

const INPUT_PATH: &str = "C:\\Users\\gabri\\OneDrive\\Documents\\Programming\\Personal\\AoC2022Rust\\input\\day01.txt";

fn read_input(path: &str) -> String {
    let content: String = fs::read_to_string(path)
        .unwrap();

    content
}

fn main()
{
    let input = read_input(INPUT_PATH);

    let mut total_calories_per_elf: Vec<i32> = input
        .split("\r\n\r\n")
        .map(|calories_per_elf| { calories_per_elf
            .split("\r\n")
            .map(|c| c.parse::<i32>().unwrap())
            .reduce(|acc, e| acc + e)
            .unwrap()
        })
        .collect();
        
    total_calories_per_elf.sort_by(|a, b| b.cmp(a));
        
    let max_calories: i32 = total_calories_per_elf[0];
    let top_three_max_calories_sum: i32 = (&total_calories_per_elf[0..3])
        .iter()
        .sum();

    println!("Part 1: {}", max_calories);
    println!("Part 2: {}", top_three_max_calories_sum);
}