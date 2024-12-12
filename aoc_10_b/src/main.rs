use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let map = read_file("practice");
    let result = get_result(map);
    if result == 81 {
        let map = read_file("input");
        println!("{}", get_result(map));
    } else {
        println!("Practice result is not valid: {}", result);
    }
}

fn read_file(filename: &'static str) -> Vec<Vec<i32>> {
    let mut map: Vec<Vec<i32>> = Vec::new();
    let file = File::open(filename).unwrap();

    for line in BufReader::new(file).lines() {
        let mut v: Vec<i32> = Vec::new();
        for char in line.unwrap().chars() {
            let position: i32 = char.to_string().parse().unwrap();
            v.push(position);
        }
        map.push(v);
    }

    map
}

fn get_result(map: Vec<Vec<i32>>) -> i32 {
    let mut result = 0;

    for i in 0..map.len() {
        for j in 0..map[i].len() {
            result += trailhead(&map, i, j, 0);
        }
    }

    result
}
fn trailhead(map: &Vec<Vec<i32>>, i: usize, j: usize, position: i32) -> i32 {
    if map[i][j] != position {
        return 0;
    }

    if position == 9 {
        return 1;
    }

    let mut th = 0;
    if i > 0 {
        th += trailhead(map, i - 1, j, position + 1);
    }

    if i < map.len() - 1 {
        th += trailhead(map, i + 1, j, position + 1);
    }

    if j > 0 {
        th += trailhead(map, i, j - 1, position + 1);
    }

    if j < map[i].len() - 1 {
        th += trailhead(map, i, j + 1, position + 1);
    }

    th
}