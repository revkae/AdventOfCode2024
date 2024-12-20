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
    let directions = [
        (1, 0),
        (-1, 0),
        (0, 1),
        (0, -1),
        (1, 1),
        (-1, -1),
        (1, -1),
        (-1, 1),
    ];
    
    let target = ["X", "M", "A", "S"];
    let mut sum = 0;
    let rows = memory.len();
    let cols = memory[0].len();
    
    for x in 0..rows {
        for y in 0..cols {
            for (dx, dy) in &directions {
                if check_direction(&memory, x, y, *dx, *dy, &target) {
                    sum += 1;
                }
            }
        }
    }
    
    println!("{}", sum);
}

fn check_direction(memory: &[Vec<String>], start_x: usize, start_y: usize, dx: isize, dy: isize, target: &[&str]) -> bool {
    let mut x = start_x as isize;
    let mut y = start_y as isize;

    for &ch in target {
        if x < 0 || x >= memory.len() as isize || y < 0 || y >= memory[0].len() as isize {
            return false;
        }
        if memory[x as usize][y as usize] != ch {
            return false;
        }
        x += dx;
        y += dy;
    }

    true
}

fn load_lines() -> Lines<BufReader<File>> {
    let path = Path::new("C:\\Users\\test\\RustroverProjects\\AdventOfCode2024\\src\\day4\\input.txt");

    let file = File::open(path).unwrap();

    BufReader::new(file).lines()
}