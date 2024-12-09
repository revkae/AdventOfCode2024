use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::path::Path;

#[allow(clippy::all)]
#[allow(unused_variables)]
#[allow(warnings)]

pub fn problem() {
    let lines = load_lines();

    let mut amount: i32 = 0;
    let mut safe_lines: Vec<Vec<i32>> = Vec::new();

    for line in lines {
        let line = line.unwrap();
        let mut levels: Vec<i32> = line.split_whitespace().map(|s| s.parse::<i32>().unwrap()).collect();
        let mut success = false;
        let mut increase = false;
        let mut decrease = false;

        success = test(&levels, &mut increase, &mut decrease);
        
        if !success {
            success = test_all(&mut levels, &mut increase, &mut decrease, 0);
        }

        if success {
            amount += 1;
            safe_lines.push(levels);
        } else {
            println!("{:?}", levels)
        }
    }

    println!("{}", amount);
}

fn load_lines() -> Lines<BufReader<File>> {
    let path = Path::new("C:\\Users\\test\\RustroverProjects\\AdventOfCode2024\\src\\day2\\input.txt");

    let file = File::open(path).unwrap();

    BufReader::new(file).lines()
}

fn test_all(levels: &mut Vec<i32>, increase: &mut bool, decrease: &mut bool, mut start: usize) -> bool {
    if start == levels.len() {
        return false;
    }
    
    let mut success = false;
    *increase = false;
    *decrease = false;
    let mut copy_levels = levels.clone();
    copy_levels.remove(start);
    
    for i in 0..copy_levels.len() - 1 {
        let first = copy_levels[i];
        let second = copy_levels[i + 1];

        success = check(first, second, increase, decrease);
        if !success {
            break;
        }
    }
    
    if !success {
        start += 1;
        
        return test_all(levels, increase, decrease, start);
    }

    success
}

fn test(levels: &[i32], increase: &mut bool, decrease: &mut bool) -> bool {
    let mut success = false;
    for i in 0..levels.len() - 1 {
        let first = levels[i];
        let second = levels[i + 1];

        success = check(first, second, increase, decrease);
        if !success {
            break;
        }
    }
    
    success
}

fn check(first: i32, second: i32, increase: &mut bool, decrease: &mut bool) -> bool {
    if (second - first).abs() <= 3 && (second - first).abs() >= 1 {
        match second.cmp(&first) {
            Ordering::Greater => {*increase = true;},
            Ordering::Less => {*decrease = true;},
            Ordering::Equal => {}
        }
        if *increase && *decrease {
            return false;
        }
        true
    } else {
        false
    }
}