use std::cmp::PartialEq;
use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;

#[derive(PartialEq)]
enum Order {
    Positive,
    Negative,
    Unknown,
}

#[derive(PartialEq)]
enum Result {
    Safe,
    Unsafe,
}

fn main() {
    let data = read_file("practice");
    let result = get_result(data);
    if result == 2 {
        let data = read_file("input");
        println!("{}", get_result(data));
    } else {
        println!("Practice result is not valid: {}", result);
    }
}

fn read_file(filename: &'static str) -> Vec<Vec<i32>> {
    let mut columns: Vec<Vec<i32>> = Vec::new();

    let file = File::open(filename).unwrap();

    for line in BufReader::new(file).lines() {
        let row = parse_line(line.unwrap());
        columns.push(row);
    }

    columns
}

fn parse_line(line: String) -> Vec<i32> {
    let mut row: Vec<i32> = Vec::new();
    let re = Regex::new(r"\d+").unwrap();

    for capture in re.captures_iter(line.as_str()) {
        let number = capture.get(0).map(|n| n.as_str().parse::<i32>().unwrap()).unwrap();
        row.push(number);
    }

    row
}

fn get_result(data: Vec<Vec<i32>>) -> i32 {
    let mut result = 0;

    for mut row in data {
        let mut prev: i32 = row.remove(0);
        let mut order = Order::Unknown;
        let mut r = Result::Safe;
        for number in row {
            let diff = number - prev;
            if diff == 0 || diff > 3 || diff < -3 {
                r = Result::Unsafe;
                break;
            }

            if order == Order::Positive && diff < 0 {
                r = Result::Unsafe;
                break;
            }

            if order == Order::Negative && diff > 0 {
                r = Result::Unsafe;
                break;
            }

            if order == Order::Unknown {
                if diff > 0 {
                    order = Order::Positive;
                } else {
                    order = Order::Negative;
                }
            }

            prev = number;
        }

        if r == Result::Safe {
            result += 1;
        }
    }

    result
}