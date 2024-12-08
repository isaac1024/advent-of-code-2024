use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let map = read_file("practice");
    let result = get_result(map);
    if result == 34 {
        let map = read_file("input");
        println!("{}", get_result(map));
    } else {
        println!("Practice result is not valid: {}", result);
    }
}

fn read_file(filename: &'static str) -> Vec<Vec<char>> {
    let mut map: Vec<Vec<char>> = Vec::new();

    let file = File::open(filename).unwrap();

    for line in BufReader::new(file).lines() {
        let l: Vec<char> = line.unwrap().chars().collect();
        map.push(l)
    }

    map
}

fn get_result(map: Vec<Vec<char>>) -> i32 {
    let mut antinodes: Vec<(i32, i32)> = Vec::new();
    let mut antennas_positions: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == '.' {
                continue;
            }

            if !antennas_positions.contains_key(&map[y][x]) {
                antennas_positions.insert(map[y][x], Vec::new());
            }

            antennas_positions.get_mut(&map[y][x]).unwrap().push((y as i32, x as i32));
        }
    }

    for (antenna, mut positions) in antennas_positions {
        while !positions.is_empty() {
            let (reference_y, reference_x) = positions.remove(0);
            for (y, x) in positions.clone() {
                if !antinodes.contains(&(reference_y, reference_x)) {
                    antinodes.push((reference_y, reference_x));
                }

                if !antinodes.contains(&(y, x)) {
                    antinodes.push((y, x));
                }

                let y_diference = y - reference_y;
                let x_diference = x - reference_x;

                let mut y_position = reference_y - y_diference;
                let mut x_position = reference_x - x_diference;
                while y_position >= 0 && y_position < map.len() as i32 && x_position >= 0 && x_position < map[0].len() as i32 {
                    if map[y_position as usize][x_position as usize] != antenna && !antinodes.contains(&(y_position, x_position)) {
                        antinodes.push((y_position, x_position));
                    }

                    y_position -= y_diference;
                    x_position -= x_diference;
                }

                y_position = y + y_diference;
                x_position = x + x_diference;
                while y_position >= 0 && y_position < map.len() as i32 && x_position >= 0 && x_position < map[0].len() as i32 {
                    if map[y_position as usize][x_position as usize] != antenna && !antinodes.contains(&(y_position, x_position)) {
                        antinodes.push((y_position, x_position));
                    }

                    y_position += y_diference;
                    x_position += x_diference;
                }
            }
        }
    }

    antinodes.iter().count() as i32
}