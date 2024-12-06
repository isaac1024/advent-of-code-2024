use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Copy, Clone, PartialEq)]
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

    fn get_char(&self) -> char {
        match self {
            Orientation::Up | Orientation::Down => '|',
            Orientation::Right | Orientation::Left => '-',
        }
    }
}

fn main() {
    let map = read_file("practice");
    let result = get_result(map);
    if result == 6 {
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
    let mut result: Vec<(i32, i32)> = Vec::new();
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
        if let Some(infinite) = infinite_loop(map.clone(), orientation, x, y) {
            if !result.contains(&infinite) {
                result.push(infinite);
            }
        }
        if map[y as usize][x as usize] == '#' {
            (x, y) = orientation.revert_position(x, y);
            orientation = orientation.change_direction();
            if map[y as usize][x as usize] != '^' {
                map[y as usize][x as usize] = '+';
            }

        } else {
            if map[y as usize][x as usize] != '^' {
                map[y as usize][x as usize] = orientation.get_char();
            }
            (x, y) = orientation.next_position(x, y);
        }
    }

    result.iter().count() as i32
}

fn infinite_loop(mut map: Vec<Vec<char>>, mut orientation: Orientation, mut x: i32, mut y: i32) -> Option<(i32, i32)> {
    let mut check_position: HashMap<(i32, i32), Vec<Orientation>> = HashMap::new();
    let (aux_x, aux_y) = orientation.next_position(x, y);
    if aux_x >= 0 && aux_x < map[0].len() as i32 && aux_y >= 0 && aux_y < map.len() as i32  && map[aux_y as usize][aux_x as usize] == '.' {
        map[aux_y as usize][aux_x as usize] = '#';
    } else {
        return None;
    }

    while x >= 0 && x < map[0].len() as i32 && y >= 0 && y < map.len() as i32 {
        if check_position.contains_key(&(x, y)) {
            if check_position.get(&(x, y)).unwrap().contains(&orientation) {
                return Some((aux_x, aux_y));
            } else {
                check_position.get_mut(&(x, y)).unwrap().push(orientation);
            }
        } else {
            let mut orientations: Vec<Orientation> = Vec::new();
            orientations.push(orientation);
            check_position.insert((x, y), orientations);
        }
        if map[y as usize][x as usize] == '#' {
            (x, y) = orientation.revert_position(x, y);
            orientation = orientation.change_direction();
        } else {
            (x, y) = orientation.next_position(x, y);
        }
    }

    None
}