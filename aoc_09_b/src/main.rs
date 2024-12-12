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
    if result == 2858 {
        let map = read_file("input");
        println!("{}", get_result(map));
    } else {
        println!("Practice result is not valid: {}", result);
    }
}

fn read_file(filename: &'static str) -> Vec<Option<i64>> {
    let mut map: Vec<(Option<i32>, i32)> = Vec::new();
    let mut i = 0;
    let mut step = Step::FileBock;

    let file = File::open(filename).unwrap();

    for line in BufReader::new(file).lines() {
        for char in line.unwrap().chars() {
            let length: i32 = char.to_string().parse().unwrap();
            if step == Step::FreeSpace {
                map.push((None, length));
                step = Step::FileBock;
            } else {
                map.push((Some(i), length));
                i += 1;
                step = Step::FreeSpace;
            }
        }
    }

    let mut j = map.len() - 1;
    while j > 0 {
        if map[j].0.is_some() {
            let mut i = 0;
            while i < j  {
                if map[i].0 == None {
                    if map[i].1 > map[j].1 {
                        map[i].1 -= map[j].1;
                        let number = map[j].0;
                        map.insert(i,(number, map[j].1));
                        j += 1;
                        map[j].0 = None;
                        break
                    } else if map[i].1 == map[j].1 {
                        map[i].0 = map[j].0;
                        map[j].0 = None;
                        break;
                    }
                }
                i += 1;
            }
        }
        j -= 1;
    }

    let mut r: Vec<Option<i64>> = Vec::new();
    for m in map {
        if let Some(i) = m.0 {
            for _ in 0..m.1 {
                r.push(Some(i as i64));
            }
        } else {
            for _ in 0..m.1 {
                r.push(None);
            }
        }
    }

    r
}

fn get_result(mut map: Vec<Option<i64>>) -> i64 {
    let mut result = 0;
    let mut i = 0;
    while !map.is_empty() {
        if let Some(number) = map.remove(0) {
            result += i * number;
        }
        i += 1;
    }

    result
}