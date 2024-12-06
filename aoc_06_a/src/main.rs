use std::fs::File;
use std::io::{BufRead, BufReader};

enum Orientation {
    Up,
    Right,
    Down,
    Left,
}

impl Orientation {
    fn change_direction(&self) -> Orientation {
        match self {
            Orientation::Up => Orientation::Right,
            Orientation::Right => Orientation::Down,
            Orientation::Down => Orientation::Left,
            Orientation::Left => Orientation::Up,
        }
    }

    fn next_position(&self, x: i32, y: i32) -> (i32, i32) {
        match self {
            Orientation::Up => (x, y-1),
            Orientation::Right => (x+1, y),
            Orientation::Down =>(x, y+1),
            Orientation::Left => (x-1, y),
        }
    }

    fn revert_position(&self, x: i32, y: i32) -> (i32, i32) {
        match self {
            Orientation::Up => (x, y+1),
            Orientation::Right => (x-1, y),
            Orientation::Down =>(x, y-1),
            Orientation::Left => (x+1, y),
        }
    }
}

fn main() {
    let map = read_file("practice");
    let result = get_result(map);
    if result == 41 {
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

fn get_result(mut map: Vec<Vec<char>>) -> i32 {
    let mut x: i32 = 0;
    let mut y: i32= 0;
    let mut orientation = Orientation::Up;

    for i in 0..map.iter().count() {
        if map[i].contains(&'^') {
            y = i as i32;

            for j in 0..map[i].iter().count() {
                if map[i][j] == '^' {
                    x = j as i32;
                    break;
                }
            }
            break;
        }
    }

    while x >= 0 && x < map[0].len() as i32 && y >= 0 && y < map.len() as i32 {
        if map[y as usize][x as usize] == '#' {
            (x, y) = orientation.revert_position(x, y);
            orientation = orientation.change_direction();
        } else {
            map[y as usize][x as usize] = 'X';
            (x, y) = orientation.next_position(x, y);
        }
    }

    map.iter().flatten().fold(0, |c, char| if char == &'X' { c + 1 } else { c })
}