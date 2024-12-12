use std::cmp::PartialEq;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(PartialEq)]
enum Step {
    FileBock,
    FreeSpace,
}

fn main() {
    let map = read_file("practice");
    let result = get_result(map);
    if result == 1928 {
        let map = read_file("input");
        println!("{}", get_result(map));
    } else {
        println!("Practice result is not valid: {}", result);
    }
}

fn read_file(filename: &'static str) -> Vec<Option<i64>> {
    let mut map: Vec<Option<i64>> = Vec::new();
    let mut i = 0;
    let mut step = Step::FileBock;

    let file = File::open(filename).unwrap();

    for line in BufReader::new(file).lines() {
        for char in line.unwrap().chars() {
            let length: i64 = char.to_string().parse().unwrap();
            for _ in 0..length {
                if step == Step::FreeSpace {
                    map.push(None)
                } else {
                    map.push(Some(i))
                }
            }

            if step == Step::FileBock {
                i += 1;
                step = Step::FreeSpace;
            } else {
                step = Step::FileBock;
            }
        }
    }

    map
}

fn get_result(mut map: Vec<Option<i64>>) -> i64 {
    let mut result = 0;
    let mut i = 0;
    while !map.is_empty() {
        if let Some(number) = map.remove(0) {
            result += i * number;
            i += 1;
        } else if let Some(last) = map.pop() {
            map.insert(0, last);
        }
    }

    result
}