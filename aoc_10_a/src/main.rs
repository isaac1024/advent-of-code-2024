use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let map = read_file("practice");
    let result = get_result(map);
    if result == 36 {
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
            result += trailhead(&map, i, j, 0, Vec::new()).iter().count() as i32;
        }
    }

    result
}

fn trailhead(map: &Vec<Vec<i32>>, i: usize, j: usize, position: i32, mut v: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    if map[i][j] != position {
        return v;
    }

    if position == 9 && !v.contains(&(i, j)) {
        v.push((i, j));
        return v;
    }

    if i > 0 {
        v = trailhead(map, i - 1, j, position + 1, v);
    }

    if i < map.len() - 1 {
        v = trailhead(map, i + 1, j, position + 1, v);
    }

    if j > 0 {
        v = trailhead(map, i, j - 1, position + 1, v);
    }

    if j < map[i].len() - 1 {
        v = trailhead(map, i, j + 1, position + 1, v);
    }

    v
}