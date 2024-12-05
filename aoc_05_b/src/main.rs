use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;

fn main() {
    let (page_ordering, page_numbers) = read_file("practice");
    let result = get_result(page_ordering, page_numbers);
    if result == 123 {
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

fn get_result(page_ordering: HashMap<i32, Vec<i32>>, page_numbers: Vec<Vec<i32>>) -> i32 {
    let numbers: Vec<i32> = page_numbers.into_iter().map(|page_number| {
        let middle_index = page_number.len()/2;
        let mut orderer_page_number = page_number.clone();
        for number in page_number.clone() {
            let index = if page_ordering.contains_key(&number) {
                let coincidences: Vec<i32> = page_ordering.get(&number)
                    .unwrap()
                    .into_iter()
                    .filter(|&n| page_number.contains(n))
                    .map(|n| { *n })
                    .collect();

                orderer_page_number.len()-coincidences.len()-1
            } else {
                orderer_page_number.len()-1
            };

            orderer_page_number[index] = number;
        }
        if orderer_page_number != page_number {
            orderer_page_number[middle_index]
        } else {
            0
        }
    }).collect();

    numbers.into_iter().sum()
}