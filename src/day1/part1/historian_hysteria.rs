use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn problem() {
    let path = Path::new("C:\\Users\\test\\RustroverProjects\\AdventOfCode2024\\src\\day1\\input.txt");

    let file = File::open(path).unwrap();

    let mut left_part: Vec<i32> = Vec::new();
    let mut right_part: Vec<i32> = Vec::new();
    for line in BufReader::new(file).lines() {
        let line = line.unwrap();
        let parts: Vec<&str> = line.split_whitespace().collect();

        let left: i32 = parts[0].parse().unwrap();
        let right: i32 = parts[1].parse().unwrap();

        left_part.push(left);
        right_part.push(right);
    }

    left_part.sort();
    right_part.sort();

    let mut distances: Vec<i32> = Vec::new();
    while !left_part.is_empty() {
        let min_left_value = left_part[0];
        let min_right_value = right_part[0];

        distances.push(min_left_value.abs_diff(min_right_value) as i32);

        left_part.remove(0);
        right_part.remove(0);
    }

    let sum = distances.iter().sum::<i32>();
    println!("{}", sum);
}