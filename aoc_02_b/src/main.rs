use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;

fn main() {
    let data = read_file("practice");
    let result = get_result(data);
    if result == 4 {
        let data = read_file("input");
        println!("{}", get_result(data));
    } else {
        println!("Practice result is not valid: {}", result);
    }
}

fn read_file(filename: &'static str) -> Vec<Vec<i32>> {
    let mut columns: Vec<Vec<i32>> = Vec::new();

    let file = File::open(filename).unwrap();

    for line in BufReader::new(file).lines() {
        let row = parse_line(line.unwrap());
        columns.push(row);
    }

    columns
}

fn parse_line(line: String) -> Vec<i32> {
    let mut row: Vec<i32> = Vec::new();
    let re = Regex::new(r"\d+").unwrap();

    for capture in re.captures_iter(line.as_str()) {
        let number = capture.get(0).map(|n| n.as_str().parse::<i32>().unwrap()).unwrap();
        row.push(number);
    }

    row
}

fn get_result(data: Vec<Vec<i32>>) -> i32 {
    let mut result = 0;

    for row in data {
        let is_safe = is_safe(row.clone());
        if is_safe {
            result += 1;
        }
    }

    result
}

fn is_safe(mut row: Vec<i32>) -> bool {
    let mut diffs: Vec<i32> = Vec::new();

    let mut prev = row.remove(0);
    for number in row {
        diffs.push(number - prev);
        prev = number;
    }

    let zeros = diffs.iter().fold(0, |n, diff| {
        if diff == &0 {
            n + 1
        } else {
            n
        }
    });

    let positives = diffs.iter().fold(0, |n, diff| {
        if diff > &0 {
            n + 1
        } else {
            n
        }
    });

    let negatives = diffs.iter().fold(0, |n, diff| {
        if diff < &0 {
            n + 1
        } else {
            n
        }
    });

    if zeros > 1 {
        return false;
    }

    if zeros == 1 {
        if negatives == 0 || positives == 0 {
            let invalid = diffs.iter().fold(0, |n, diff| {
                if diff < &-3 || diff > &3 {
                    n + 1
                } else {
                    n
                }
            });

            return invalid == 0;
        } else {
            return false;
        }
    }

    if positives > 1 && negatives > 1 {
        return false;
    }

    if positives == 0 || negatives == 0 {
        let invalid = diffs.iter().fold(0, |n, diff| {
            if diff < &-3 || diff > &3 {
                n + 1
            } else {
                n
            }
        });

        if invalid > 1 {
            return false;
        };

        if invalid == 0 {
            return true;
        }

        if diffs[0] > 3 || diffs[0] < -3 {
            return true;
        }

        if diffs[diffs.len()-1] > 3 || diffs[diffs.len()-1] < -3 {
            return true;
        }

        return false;
    }

    if positives == 1 {
        for i in 0..diffs.len() {
            if diffs[i] < 0 {
                continue;
            }

            if i == 0 || i == diffs.len()-1 {
                let mut cloned_diffs = diffs.clone();
                cloned_diffs.remove(i);

                let invalid = cloned_diffs.iter().fold(0, |n, diff| {
                    if diff < &-3 {
                        n + 1
                    } else {
                        n
                    }
                });

                if invalid == 0 {
                    return true;
                }
            }

            if i > 0 {
                let mut cloned_diffs = diffs.clone();
                let aux = cloned_diffs.remove(i);
                cloned_diffs[i-1] += aux;

                if cloned_diffs[i-1] < 0 {
                    let invalid = cloned_diffs.iter().fold(0, |n, diff| {
                        if diff < &-3 {
                            n + 1
                        } else {
                            n
                        }
                    });

                    if invalid == 0 {
                        return true;
                    }
                }
            }

            if i < diffs.len()-1 {
                let mut cloned_diffs = diffs.clone();
                let aux = cloned_diffs.remove(i);
                cloned_diffs[i] += aux;

                if cloned_diffs[i] < 0 {
                    let invalid = cloned_diffs.iter().fold(0, |n, diff| {
                        if diff < &-3 {
                            n + 1
                        } else {
                            n
                        }
                    });

                    if invalid == 0 {
                        return true;
                    }
                }
            }
        }

        return false;
    } else if negatives == 1 {
        for i in 0..diffs.len() {
            if diffs[i] > 0 {
                continue;
            }

            if i == 0 || i == diffs.len()-1 {
                let mut cloned_diffs = diffs.clone();
                cloned_diffs.remove(i);

                let invalid = cloned_diffs.iter().fold(0, |n, diff| {
                    if diff > &3 {
                        n + 1
                    } else {
                        n
                    }
                });

                if invalid == 0 {
                    return true;
                }
            }

            if i > 0 {
                let mut cloned_diffs = diffs.clone();
                let aux = cloned_diffs.remove(i);
                cloned_diffs[i-1] += aux;

                if cloned_diffs[i-1] > 0 {
                    let invalid = cloned_diffs.iter().fold(0, |n, diff| {
                        if diff > &3 {
                            n + 1
                        } else {
                            n
                        }
                    });

                    if invalid == 0 {
                        return true;
                    }
                }
            }

            if i < diffs.len()-1 {
                let mut cloned_diffs = diffs.clone();
                let aux = cloned_diffs.remove(i);
                cloned_diffs[i] += aux;

                if cloned_diffs[i] > 0 {
                    let invalid = cloned_diffs.iter().fold(0, |n, diff| {
                        if diff > &3 {
                            n + 1
                        } else {
                            n
                        }
                    });

                    if invalid == 0 {
                        return true;
                    }
                }
            }
        }

        return false;
    }

    true
}