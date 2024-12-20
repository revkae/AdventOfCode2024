use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::path::Path;
use std::str::Chars;

pub fn problem() {
    let lines: Lines<BufReader<File>> = load_lines();

    let mut memory: Vec<Vec<String>> = Vec::new();
    for line in lines {
        match line {
            Ok(content) => {
                let char_vec = content.chars().map(|c| c.to_string()).collect();
                memory.push(char_vec);
            },
            Err(e) => println!("Error: {}", e)
        }
    }
    let target = ["M", "A", "S"];
    let mut sum = 0;
    let rows = memory.len();
    let cols = memory[0].len();
    let directions = [
        (1, 1),
        (1, -1),
    ];

    let mut all_As: Vec<(usize, usize)> = Vec::new();
    for x in 0..rows {
        for y in 0..cols {
            if &memory[x][y] == "A" {
                all_As.push((x, y));
            }
        }
    }

    for a in all_As {
        if check_mas(&memory, a) {
            sum += 1;
        }
    }

    println!("{}", sum);
}

fn check_mas(memory: &[Vec<String>], current_pos: (usize, usize)) -> bool {
    let allowed = ["MAS", "SAM"];
    let first = check_direction(memory, (current_pos.0.wrapping_sub(1), current_pos.1 + 1), (1, -1));
    let second = check_direction(memory, (current_pos.0.wrapping_sub(1), current_pos.1.wrapping_sub(1)), (1, 1));

    if (first == allowed[0] || first == allowed[1]) && (second == allowed[0] || second == allowed[1]) {
        return true;
    }

    false
}

fn check_direction(memory: &[Vec<String>], current_pos: (usize, usize), direction: (isize, isize)) -> String {
    let mut x = current_pos.0 as isize;
    let mut y = current_pos.1 as isize;
    let mut word = String::new();

    for i in 0..3 {
        if x < 0 || x >= memory.len() as isize || y < 0 || y >= memory[0].len() as isize {
            return word;
        }
        let ch = memory[x as usize][y as usize].clone();
        word.push_str(&ch);
        x += direction.0;
        y += direction.1;
    }

    word
}

fn load_lines() -> Lines<BufReader<File>> {
    let path = Path::new("C:\\Users\\test\\RustroverProjects\\AdventOfCode2024\\src\\day4\\input.txt");

    let file = File::open(path).unwrap();

    BufReader::new(file).lines()
}