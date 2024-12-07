use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;

fn main() {
    let map = read_file("practice");
    let result = get_result(map);
    if result == 11387 {
        let map = read_file("input");
        println!("{}", get_result(map));
    } else {
        println!("Practice result is not valid: {}", result);
    }
}

fn read_file(filename: &'static str) -> Vec<(u64, Vec<u64>)> {
    let file = File::open(filename).unwrap();
    BufReader::new(file).lines().map(|line| parse_line(line.unwrap())).collect()
}

fn parse_line(line: String) -> (u64, Vec<u64>) {
    let mut numbers: Vec<u64> = Vec::new();
    let re = Regex::new(r"\d+").unwrap();

    for capture in re.captures_iter(line.as_str()) {
        let number = capture.get(0).map(|n| n.as_str().parse::<u64>().unwrap()).unwrap();
        numbers.push(number);
    }

    (numbers.remove(0), numbers)
}

fn get_result(numbers: Vec<(u64, Vec<u64>)>) -> u64 {
    numbers.into_iter().fold(0, |c, (r, mut n)| if check_operator(r, n.remove(0), n) { c + r } else { c })
}

fn check_operator(result: u64, carry: u64, mut numbers: Vec<u64>) -> bool {
    if numbers.is_empty() {
        return result == carry;
    }

    let n = numbers.remove(0);
    check_operator(result, carry * n, numbers.clone())
        || check_operator(result, carry + n, numbers.clone())
        || check_operator(result, format!("{}{}", carry, n).parse::<u64>().unwrap(), numbers)
}