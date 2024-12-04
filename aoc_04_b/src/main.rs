use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;

fn main() {
    let result = get_result("practice");
    if result == 9 {
        println!("{}", get_result("input"));
    } else {
        println!("Practice result is not valid: {}", result);
    }
}

fn get_result(filename: &'static str) -> i32 {
    get_lines(filename).into_iter().fold(0, |c, l| c + count_xmas(l))
}

fn get_lines(filename: &'static str) -> Vec<String> {
    let mut lines: Vec<String> = Vec::new();
    let mut i = 0;

    let file = File::open(filename).unwrap();

    for line in BufReader::new(file).lines() {
        let l = line.unwrap();
        let line_size = l.len();
        for (index, char) in l.char_indices() {
            if index + 2 < line_size {
                if lines.len() <= index + i * (line_size - 2) {
                    lines.push(char.to_string())
                } else {
                    lines.get_mut(index + i * (line_size - 2)).unwrap().push(char)
                }
            }

            if index > 0 && index + 1 < line_size {
                lines.get_mut(index - 1 + i * (line_size - 2)).unwrap().push(char)
            }

            if index > 1 {
                lines.get_mut(index - 2 + i * (line_size - 2)).unwrap().push(char)
            }

            if i > 0 {
                if index + 2 < line_size {
                    if lines.len() <= index + (i - 1) * (line_size - 2) {
                        lines.push(char.to_string())
                    } else {
                        lines.get_mut(index + (i - 1) * (line_size - 2)).unwrap().push(char)
                    }
                }

                if index > 0 && index + 1 < line_size {
                    lines.get_mut(index - 1 + (i - 1) * (line_size - 2)).unwrap().push(char)
                }

                if index > 1 {
                    lines.get_mut(index - 2 + (i - 1) * (line_size - 2)).unwrap().push(char)
                }
            }

            if i > 1 {
                if index + 2 < line_size {
                    if lines.len() <= index + (i - 2) * (line_size - 2) {
                        lines.push(char.to_string())
                    } else {
                        lines.get_mut(index + (i - 2) * (line_size - 2)).unwrap().push(char)
                    }
                }

                if index > 0 && index + 1 < line_size {
                    lines.get_mut(index - 1 + (i - 2) * (line_size - 2)).unwrap().push(char)
                }

                if index > 1 {
                    lines.get_mut(index - 2 + (i - 2) * (line_size - 2)).unwrap().push(char)
                }
            }
        }

        i += 1;
    }

    lines
}

fn count_xmas(line: String) -> i32 {
    let re1 = Regex::new(r"M.S.A.M.S").unwrap();
    let re2 = Regex::new(r"M.M.A.S.S").unwrap();
    let re3 = Regex::new(r"S.M.A.S.M").unwrap();
    let re4 = Regex::new(r"S.S.A.M.M").unwrap();

    let v1: i32 = re1.find_iter(line.as_str()).collect::<Vec<_>>().len().try_into().unwrap();
    let v2: i32 = re2.find_iter(line.as_str()).collect::<Vec<_>>().len().try_into().unwrap();
    let v3: i32 = re3.find_iter(line.as_str()).collect::<Vec<_>>().len().try_into().unwrap();
    let v4: i32 = re4.find_iter(line.as_str()).collect::<Vec<_>>().len().try_into().unwrap();

    v1 + v2 + v3 + v4
}