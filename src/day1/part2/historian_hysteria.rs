use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[allow(clippy::all)]
#[allow(unused_variables)]
#[allow(warnings)]

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
    for x in left_part {
        distances.push(x * how_many(x, &right_part));
    }

    let sum = distances.iter().sum::<i32>();
    println!("{}", sum);
}

fn how_many(x: i32, right_part: &Vec<i32>) -> i32 {
    let mut amount = 0;
    for right in right_part {
        if *right == x {
            amount += 1;
        }
    }

    amount
}