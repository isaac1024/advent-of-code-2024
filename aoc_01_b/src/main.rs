use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;

fn main() {
    let (left_numbers, right_numbers) = read_file("practice");
    let result = get_result(left_numbers, right_numbers);
    if result == 31 {
        let (left_numbers, right_numbers) = read_file("input");
        println!("{}", get_result(left_numbers, right_numbers));
    } else {
        println!("Practice result is not valid: {}", result);
    }
}

fn read_file(filename: &'static str) -> (HashMap<i32, i32>, HashMap<i32, i32>) {
    let mut left_numbers: HashMap<i32, i32> = HashMap::new();
    let mut right_numbers: HashMap<i32, i32> = HashMap::new();

    let practice_file = File::open(filename).unwrap();

    for line in BufReader::new(practice_file).lines() {
        let (left_number, right_number) = parse_line(line.unwrap());
        if left_numbers.contains_key(&left_number) {
            left_numbers.entry(left_number).and_modify(|n| *n += 1);
        } else { 
            left_numbers.insert(left_number, 1);
        }
        if right_numbers.contains_key(&right_number) {
            right_numbers.entry(right_number).and_modify(|n| *n += 1);
        } else {
            right_numbers.insert(right_number, 1);
        }
    }

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

fn get_result(left: HashMap<i32, i32>, right: HashMap<i32, i32>) -> i32 {
    let mut result = 0;

    for (number, times) in left {
        if right.contains_key(&number) {
            result += number * times * right[&number];
        }
    }

    result
}