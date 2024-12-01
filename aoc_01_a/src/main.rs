use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;

fn main() {
    let (left_numbers, right_numbers) = read_file("practice");
    let result = get_result(left_numbers, right_numbers);
    if result == 11 {
        let (left_numbers, right_numbers) = read_file("input");
        println!("{}", get_result(left_numbers, right_numbers));
    } else {
        println!("Practice result is not valid: {}", result);
    }
}

fn read_file(filename: &'static str) -> (Vec<i32>, Vec<i32>) {
    let mut left_numbers: Vec<i32> = Vec::new();
    let mut right_numbers: Vec<i32> = Vec::new();

    let practice_file = File::open(filename).unwrap();

    for line in BufReader::new(practice_file).lines() {
        let (left_number, right_number) = parse_line(line.unwrap());
        left_numbers.push(left_number);
        right_numbers.push(right_number)
    }

    left_numbers.sort();
    right_numbers.sort();

    (left_numbers, right_numbers)
}

fn parse_line(line: String) -> (i32, i32) {
    let re = Regex::new(r"^(?<left>\d+)\W+(?<right>\d+)$").unwrap();

    re.captures(line.as_str()).map(|m| {
        let left = m.name("left")
            .map(|n| n.as_str().parse::<i32>().unwrap())
            .unwrap();
        let right = m.name("right")
            .map(|n| n.as_str().parse::<i32>().unwrap())
            .unwrap();
        (left, right)
    }).unwrap()
}

fn get_result(left: Vec<i32>, right: Vec<i32>) -> i32 {
    let mut result = 0;

    for i in 0..left.len() {
        if right[i] > left[i] {
            result += right[i] - left[i];
        } else {
            result += left[i] - right[i];
        }
    }

    result
}