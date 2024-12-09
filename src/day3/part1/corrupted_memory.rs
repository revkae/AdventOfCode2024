use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::path::Path;

pub fn problem() {
    let lines: Lines<BufReader<File>> = load_lines();
    let memory: String = lines
        .filter_map(Result::ok)
        .collect::<Vec<String>>()
        .join("");
    
    for (index, _) in memory.match_indices("mul") {
        println!("Found '{}' at index {}.", "mul", index);
    }
}

fn load_lines() -> Lines<BufReader<File>> {
    let path = Path::new("C:\\Users\\test\\RustroverProjects\\AdventOfCode2024\\src\\day3\\input.txt");

    let file = File::open(path).unwrap();

    BufReader::new(file).lines()
}