use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::path::Path;
use std::str::Chars;
use regex::Regex;

pub fn problem() {
    let lines: Lines<BufReader<File>> = load_lines();
    let mut memory: String = lines
        .map_while(Result::ok)
        .collect::<Vec<String>>()
        .join("");

    let mut sum = 0;
    println!("{}", memory.len());
    let re = Regex::new(r"don't\(\).*?do\(\)").unwrap();
    
    memory = re.replace_all(memory.as_str(), "").parse().unwrap();
    
    println!("{}", memory.len());

    for (index, _) in memory.match_indices("mul(") {
        let start = memory.chars().nth(index + 2).unwrap();
        let result = check(start, &memory, index);
        if result.0 {
            sum += (result.1.parse::<i32>().unwrap() * result.2.parse::<i32>().unwrap());
        }
    }
    println!("{}", sum);
}

fn check(start: char, memory: &str, index: usize) -> (bool, String, String) {
    let mut count = 4;
    let mut control = memory.chars().nth(index + count).unwrap();
    let mut firstNumber = String::new();
    let mut secondNumber = String::new();
    let mut foundFirstNumber = false;
    let mut foundSecondNumber = false;
    let mut foundComma = false;
    while true {
        if foundComma {
            break;
        }

        if !foundFirstNumber && control.is_numeric() {
            foundFirstNumber = true;
            firstNumber.push(control);
        }
        else if !foundFirstNumber && !control.is_numeric() {
            break;
        }
        else if foundFirstNumber && control.is_numeric() {
            firstNumber.push(control);
        }
        else if foundFirstNumber && control == ',' {
            foundComma = true;
        } else {
            break;
        }

        count += 1;
        control = memory.chars().nth(index + count).unwrap();
    }

    if !foundComma {
        return (false, firstNumber, secondNumber);
    }

    let mut foundPar = false;
    while true {
        if foundPar {
            break;
        }

        if !foundSecondNumber && control.is_numeric() {
            foundSecondNumber = true;
            secondNumber.push(control);
        }
        else if !foundSecondNumber && !control.is_numeric() {
            break;
        }
        else if foundSecondNumber && control.is_numeric() {
            secondNumber.push(control);
        }
        else if foundSecondNumber && control == ')' {
            foundPar = true;
        } else {
            break;
        }

        count += 1;
        control = memory.chars().nth(index + count).unwrap();
    }

    if foundComma && foundPar {
        return (true, firstNumber, secondNumber);
    }

    (false, firstNumber, secondNumber)
}

fn load_lines() -> Lines<BufReader<File>> {
    let path = Path::new("C:\\Users\\test\\RustroverProjects\\AdventOfCode2024\\src\\day3\\input.txt");

    let file = File::open(path).unwrap();

    BufReader::new(file).lines()
}