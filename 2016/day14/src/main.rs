use std::collections::{HashMap, HashSet};

use md5::{Digest, Md5};

fn main() {
    let input = include_str!("../input");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let mut hasher = Md5::new();
    let mut num_keys = 0;
    let mut index = 0;

    let prefix = input.trim().as_bytes().to_vec();

    while num_keys < 64 {
        let mut temp = prefix.clone();
        temp.extend_from_slice(index.to_string().as_bytes());

        hasher.update(temp);

        let current = format!("{:x}", hasher.finalize_reset());
        if let Some(c) = current
            .chars()
            .collect::<Vec<char>>()
            .windows(3)
            .filter_map(|w| {
                if w[0] == w[1] && w[1] == w[2] {
                    Some(w[0])
                } else {
                    None
                }
            })
            .next()
        {
            let mut is_key = false;
            for i in 1..=1000 {
                let mut temp = prefix.clone();
                temp.extend_from_slice((index + i).to_string().as_bytes());

                hasher.update(temp);

                let current = format!("{:x}", hasher.finalize_reset());
                if current
                    .chars()
                    .collect::<Vec<char>>()
                    .windows(5)
                    .any(|w| c == w[0] && HashSet::<char>::from_iter(w.to_vec()).len() == 1)
                {
                    is_key = true;
                    break;
                }
            }

            if is_key {
                num_keys += 1;
            }
        }

        index += 1;
    }

    index - 1
}

fn hash_of(input: String, known: &mut HashMap<String, String>) -> String {
    if known.contains_key(&input) {
        return known.get(&input).unwrap().clone();
    }

    let mut hasher = Md5::new();
    let mut result = input.clone();

    for _ in 0..2017 {
        hasher.update(result.as_bytes());
        result = format!("{:x}", hasher.finalize_reset());
        // println!("{result}");
    }

    known.insert(input, result.clone());

    result
}

fn part2(input: &str) -> usize {
    let mut num_keys = 0;
    let mut index = 0;
    let mut known: HashMap<String, String> = HashMap::new();

    let prefix = input.trim().to_string();

    while num_keys < 64 {
        let mut temp = prefix.clone();
        temp.push_str(&index.to_string());

        let current = hash_of(temp, &mut known);
        if let Some(c) = current
            .chars()
            .collect::<Vec<char>>()
            .windows(3)
            .filter_map(|w| {
                if w[0] == w[1] && w[1] == w[2] {
                    Some(w[0])
                } else {
                    None
                }
            })
            .next()
        {
            let mut is_key = false;
            for i in 1..=1000 {
                let mut temp = prefix.clone();
                temp.push_str(&(index + i).to_string());

                let current = hash_of(temp, &mut known);
                if current
                    .chars()
                    .collect::<Vec<char>>()
                    .windows(5)
                    .any(|w| c == w[0] && HashSet::<char>::from_iter(w.to_vec()).len() == 1)
                {
                    is_key = true;
                    break;
                }
            }

            if is_key {
                num_keys += 1;
            }
        }

        index += 1;
    }

    index - 1
}
