use std::{fs, collections::HashSet};

const INPUT_PATH: &str = "C:\\Users\\gabri\\OneDrive\\Documents\\Programming\\Personal\\advent-of-code-2022\\input\\day09.txt";
const MAP_SIZE: u32 = 2000;

struct Knot {
    x: u32,
    y: u32
}

impl Knot {
    fn update_pos(&mut self, dir: char) {
        match dir {
            'U' => self.y += 1,
            'D' => self.y -= 1,
            'L' => self.x -= 1,
            'R' => self.x += 1,
            _   => panic!("Invalid direction!")
        };
    }

    fn is_adjacent(&self, k2: &Knot) -> bool {
           (self.x.abs_diff(k2.x) <= 1) 
        && (self.y.abs_diff(k2.y) <= 1)
    }

    fn is_align(&self, k2: &Knot) -> bool {
        (self.x == k2.x) || (self.y == k2.y)
    }
}

fn read_input(path: &str) -> String {
    let content: String = fs::read_to_string(path).unwrap();

    content
}

fn parse_line(line: &str) -> (char, u32) {
    let mut line = line.split(" ");
    let dir = line
        .next()
        .unwrap()
        .chars()
        .next()
        .unwrap();
    let amount = line
        .next()
        .unwrap()
        .parse::<u32>().unwrap();

    (dir, amount)
}

fn main() {
    let input = read_input(INPUT_PATH);

    // Part 1
    let mut head = Knot { x: MAP_SIZE / 2, y: MAP_SIZE / 2 };
    let mut tail = Knot { x: MAP_SIZE / 2, y: MAP_SIZE / 2 };
    let mut visited: HashSet<(u32, u32)> = HashSet::from([(tail.x, tail.y)]);

    for line in input.lines() {
        let (dir, amount) = parse_line(line);

        for _ in 0..amount {
            let head_last_pos = (head.x, head.y);
            head.update_pos(dir);

            if !head.is_adjacent(&tail) {
                tail.x = head_last_pos.0;
                tail.y = head_last_pos.1;
                visited.insert((tail.x, tail.y));
            }
        }
    }

    let total_visited = visited.len();
    println!("Part 1: {}", total_visited);

    // Part 2
    let mut rope = (1..=10)
        .map(|_| Knot { x: MAP_SIZE / 2, y: MAP_SIZE / 2 })
        .collect::<Vec<_>>();
    let mut visited: HashSet<(u32, u32)> = HashSet::from([(
        rope.last().unwrap().x, 
        rope.last().unwrap().y
    )]);

    for line in input.lines() {
        let (dir, amount) = parse_line(line);

        for _ in 0..amount {
            rope[0].update_pos(dir);

            for i in 1..rope.len() {
                if !rope[i].is_adjacent(&rope[i-1]) {
                    if rope[i].is_align(&rope[i-1]) {
                        if rope[i].x != rope[i-1].x {
                            let x_dir = if rope[i].x < rope[i-1].x { 'R' } else { 'L' };
                            rope[i].update_pos(x_dir);
                        } else {
                            let y_dir = if rope[i].y < rope[i-1].y { 'U' } else { 'D' };
                            rope[i].update_pos(y_dir);
                        }
                    } else {
                        let x_dir = if rope[i].x < rope[i-1].x { 'R' } else { 'L' };
                        let y_dir = if rope[i].y < rope[i-1].y { 'U' } else { 'D' };

                        rope[i].update_pos(x_dir);
                        rope[i].update_pos(y_dir);
                    }
                }
            }

            visited.insert((
                rope.last().unwrap().x, 
                rope.last().unwrap().y
            ));
        }
    }

    let total_visited = visited.len();
    println!("Part 2: {}", total_visited);
}