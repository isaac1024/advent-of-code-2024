use std::fs;
use regex::Regex;

fn main() {
    let data = read_file("practice");
    let result = get_result(data);
    if result == 48 {
        let data = read_file("input");
        println!("{}", get_result(data));
    } else {
        println!("Practice result is not valid: {}", result);
    }
}

fn read_file(filename: &'static str) -> Vec<(i32, i32)> {
    let file_string = fs::read_to_string(filename).unwrap();
    parse_string(file_string)
}

fn parse_string(line: String) -> Vec<(i32, i32)> {
    let mut row: Vec<(i32, i32)> = Vec::new();
    let re = Regex::new(r"mul\((?<first>\d+),(?<second>\d+)\)|do(n't)?\(\)").unwrap();

    let mut enabled = true;
    for capture in re.captures_iter(line.as_str()) {
        let captured = capture.get(0).unwrap().as_str();
        if captured == "do()" {
            enabled = true;
        } else if captured == "don't()" {
            enabled = false
        } else if enabled == true {
            let first = capture.name("first").unwrap().as_str().parse::<i32>().unwrap();
            let second = capture.name("second").unwrap().as_str().parse::<i32>().unwrap();
            row.push((first, second));
        }
    }

    row
}

fn get_result(data: Vec<(i32, i32)>) -> i32 {
    data.iter().fold(0, |c, t| c + t.0 * t.1 )
}