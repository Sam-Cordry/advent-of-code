use std::collections::HashMap;

use md5::{Digest, Md5};

fn main() {
    let input = include_str!("../input");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> String {
    let mut result = String::new();
    let mut hasher = Md5::new();

    let mut count = 0;
    let original = input.trim().as_bytes().to_vec();

    while result.len() < 8 {
        let mut temp = original.clone();
        temp.extend_from_slice(count.to_string().as_bytes());

        hasher.update(temp);

        let current = format!("{:x}", hasher.finalize_reset());
        if &current[..5] == "00000" {
            result.push(current.chars().nth(5).unwrap());
        }

        count += 1;
    }

    result
}

fn part2(input: &str) -> String {
    let mut result: HashMap<usize, char> = HashMap::new();
    let mut hasher = Md5::new();

    let mut count = 0;
    let original = input.trim().as_bytes().to_vec();

    while result.len() < 8 {
        let mut temp = original.clone();
        temp.extend_from_slice(count.to_string().as_bytes());

        hasher.update(temp);

        let current = format!("{:x}", hasher.finalize_reset());
        if &current[..5] == "00000" {
            if let Ok(idx) = current.chars().nth(5).unwrap().to_string().parse::<usize>() {
                if idx < 8 && !result.contains_key(&idx) {
                    result.insert(idx, current.chars().nth(6).unwrap());
                }
            }
        }

        count += 1;
    }

    let mut temp = result
        .iter()
        .map(|e| (*e.0, *e.1))
        .collect::<Vec<(usize, char)>>();
    temp.sort();
    temp.iter().map(|(_, c)| c).collect::<String>()
}
