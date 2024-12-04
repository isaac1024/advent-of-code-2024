use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;

fn main() {
    let result = get_result("practice");
    if result == 18 {
        println!("{}", get_result("input"));
    } else {
        println!("Practice result is not valid: {}", result);
    }
}

fn get_result(filename: &'static str) -> i32 {
    let file = File::open(filename).unwrap();

    BufReader::new(file).lines().fold(0, |c, l| c + count_xmas(l.unwrap()))
        + vertical_lines(filename).into_iter().fold(0, |c, l| c + count_xmas(l))
        + diagonal_lines(filename).into_iter().fold(0, |c, l| c + count_xmas(l))
        + reverse_diagonal_lines(filename).into_iter().fold(0, |c, l| c + count_xmas(l))
}

fn vertical_lines(filename: &'static str) -> Vec<String> {
    let mut lines: Vec<String> = Vec::new();

    let file = File::open(filename).unwrap();

    for line in BufReader::new(file).lines() {
        for (index, char) in line.unwrap().char_indices() {
            if lines.len() <= index {
                lines.push(char.to_string())
            } else {
                lines.get_mut(index).unwrap().push(char)
            }
        }
    }

    lines
}

fn diagonal_lines(filename: &'static str) -> Vec<String> {
    let mut lines: Vec<String> = Vec::new();
    let mut offset = 0;

    let file = File::open(filename).unwrap();

    for line in BufReader::new(file).lines() {
        for (index, char) in line.unwrap().char_indices() {
            if lines.len() <= index + offset {
                lines.push(char.to_string())
            } else {
                lines.get_mut(index + offset).unwrap().push(char)
            }
        }
        offset += 1;
    }

    lines
}

fn reverse_diagonal_lines(filename: &'static str) -> Vec<String> {
    let mut lines: Vec<String> = Vec::new();
    let mut offset = 0;

    let file = File::open(filename).unwrap();

    for line in BufReader::new(file).lines() {
        let reverse_line: String = line.unwrap().chars().rev().collect();
        for (index, char) in reverse_line.char_indices() {
            if lines.len() <= index + offset {
                lines.push(char.to_string())
            } else {
                lines.get_mut(index + offset).unwrap().push(char)
            }
        }
        offset += 1;
    }

    lines
}

fn count_xmas(line: String) -> i32 {
    let re1 = Regex::new(r"XMAS").unwrap();
    let re2 = Regex::new(r"SAMX").unwrap();

    let first_value: i32 = re1.find_iter(line.as_str()).collect::<Vec<_>>().len().try_into().unwrap();
    let second_value: i32 = re2.find_iter(line.as_str()).collect::<Vec<_>>().len().try_into().unwrap();

    first_value + second_value
}