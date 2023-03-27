use std::{fs};

const INPUT_PATH: &str = "C:\\Users\\gabri\\OneDrive\\Documents\\Programming\\Personal\\advent-of-code-2022\\input\\day08.txt";

fn read_input(path: &str) -> String {
    let content: String = fs::read_to_string(path).unwrap();

    content
}

fn main() {
    let input = read_input(INPUT_PATH);

    let trees = input
        .lines()
        .map(|l| {
            l
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect::<Vec<u32>>()
        })
        .collect::<Vec<_>>();

    let trees_len = trees.len();

    let mut visible: Vec<Vec<bool>> = Vec::new();
    for _ in 0..trees_len {
        let mut row: Vec<bool> = Vec::new();
        for _ in 0..trees_len {
            row.push(false);
        }
        visible.push(row);
    }

    // Set the outside trees visibility to true.
    for i in 0..trees_len {
        visible[0][i] = true;
        visible[trees_len-1][i] = true;
        visible[i][0] = true;
        visible[i][trees_len-1] = true;
    }

    // Check visibility from the east and west direction.
    for i in 1..trees_len {
        let mut west_tallest = trees[i][0];
        let mut east_tallest = trees[i][trees_len-1];
        for j in 1..trees_len {
            if trees[i][j] > west_tallest {
                west_tallest = trees[i][j];
                visible[i][j] = true;
            }
            if trees[i][trees_len-j-1] > east_tallest {
                east_tallest = trees[i][trees_len-j-1];
                visible[i][trees_len-j-1] = true;
            }
        }
    }

    // Check visibility from the north and south direction.
    for j in 1..trees_len {
        let mut north_tallest = trees[0][j];
        let mut south_tallest = trees[trees_len-1][j];
        for i in 1..trees_len {
            if trees[i][j] > north_tallest {
                north_tallest = trees[i][j];
                visible[i][j] = true;
            }
            if trees[trees_len-i-1][j] > south_tallest {
                south_tallest = trees[trees_len-i-1][j];
                visible[trees_len-i-1][j] = true;
            }
        }
    }

    let total_visible_trees = visible
        .into_iter()
        .map(|row| {
            row.into_iter().filter(|&b| b).count()
        })
        .reduce(|acc, e| acc + e)
        .unwrap();

    println!("Part 1: {}", total_visible_trees);

    // Part 2
    let mut scenic_score: Vec<Vec<u64>> = Vec::new();
    for _ in 0..trees_len {
        let mut row: Vec<u64> = Vec::new();
        for _ in 0..trees_len {
            row.push(1);
        }
        scenic_score.push(row);
    }

    // Set scenic score of edge trees to 0.
    for i in 0..trees_len {
        scenic_score[i][0] = 0;
        scenic_score[i][trees_len-1] = 0;
        scenic_score[0][i] = 0;
        scenic_score[trees_len-1][i];
    }

    let mut largest_scenic_score = 0;

    for i in 1..trees_len-1 {
        for j in 1..trees_len-1 {
            let mut is_largest = true;
            let height = trees[i][j];

            for k in (0..=j-1).rev() {
                if height <= trees[i][k] {
                    scenic_score[i][j] *= k.abs_diff(j) as u64;
                    is_largest = false;
                    break;
                }
            }
            
            if is_largest {
                scenic_score[i][j] *= j as u64;
            }

            is_largest = true;

            for k in j+1..trees_len {
                if height <= trees[i][k] {
                    scenic_score[i][j] *= k.abs_diff(j) as u64;
                    is_largest = false;
                    break;
                }
            }
            
            if is_largest {
                scenic_score[i][j] *= (trees_len - j - 1) as u64;
            }

            is_largest = true;

            for k in (0..=i-1).rev() {
                if height <= trees[k][j] {
                    scenic_score[i][j] *= k.abs_diff(i) as u64;
                    is_largest = false;
                    break;
                }
            }
            
            if is_largest {
                scenic_score[i][j] *= i as u64;
            }

            is_largest = true;

            for k in i+1..trees_len {
                if height <= trees[k][j] {
                    scenic_score[i][j] *= k.abs_diff(i) as u64;
                    is_largest = false;
                    break;
                }
            }
            
            if is_largest {
                scenic_score[i][j] *= (trees_len - i - 1) as u64;
            }

            largest_scenic_score = largest_scenic_score.max(scenic_score[i][j]);
        }
    }

    println!("Part 2: {}", largest_scenic_score);
}