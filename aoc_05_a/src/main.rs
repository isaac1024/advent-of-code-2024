use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;

fn main() {
    let (page_ordering, page_numbers) = read_file("practice");
    let result = get_result(page_ordering, page_numbers);
    if result == 143 {
        let (page_ordering, page_numbers) = read_file("input");
        println!("{}", get_result(page_ordering, page_numbers));
    } else {
        println!("Practice result is not valid: {}", result);
    }
}

fn read_file(filename: &'static str) -> (HashMap<i32, Vec<i32>>, Vec<Vec<i32>>) {
    let mut page_ordering: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut page_numbers: Vec<Vec<i32>> = Vec::new();

    let re_page_ordering = Regex::new(r"^(?<left>\d+)\|(?<right>\d+)$").unwrap();
    let re_page_numbers = Regex::new(r"(?<number>\d+)").unwrap();

    let file = File::open(filename).unwrap();

    for line in BufReader::new(file).lines() {
        let l = line.unwrap();
        if re_page_ordering.is_match(l.as_str()) {
            let (left, right) = re_page_ordering.captures(l.as_str()).map(|m| {
                let left = m.name("left")
                    .map(|n| n.as_str().parse::<i32>().unwrap())
                    .unwrap();
                let right = m.name("right")
                    .map(|n| n.as_str().parse::<i32>().unwrap())
                    .unwrap();
                (left, right)
            }).unwrap();

            if page_ordering.contains_key(&left) {
                page_ordering.get_mut(&left).unwrap().push(right);
            } else {
                page_ordering.insert(left, vec![right]);
            }
        } else if re_page_numbers.is_match(l.as_str()) {
            let pages: Vec<i32> = re_page_numbers.captures_iter(l.as_str())
                .filter_map(|n| n.name("number").and_then(|n| n.as_str().parse::<i32>().ok()))
                .collect();
            page_numbers.push(pages);
        }
    }

    (page_ordering, page_numbers)
}

fn get_result(page_ordering: HashMap<i32, Vec<i32>>, mut page_numbers: Vec<Vec<i32>>) -> i32 {
    let numbers: Vec<i32> = page_numbers.iter_mut().map(|page_number| {
        let mut prev = page_number.remove(0);
        for number in &mut *page_number {
            if !page_ordering.contains_key(&prev) || !page_ordering.get(&prev).unwrap().contains(number) {
                return 0;
            }
            prev = *number;
        }
        *page_number.get(page_number.len() / 2 - 1).unwrap()
    }).collect();

    numbers.into_iter().sum()
}