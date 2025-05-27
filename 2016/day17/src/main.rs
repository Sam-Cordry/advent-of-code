use md5::{Digest, Md5};
use std::collections::VecDeque;

fn main() {
    let input = include_str!("../input");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn get_next(current: ((usize, usize), String), passcode: &str) -> Vec<((usize, usize), String)> {
    let mut next = vec![];

    let hash = {
        let mut hasher = Md5::new();
        let mut to_hash = passcode.to_string();
        to_hash.push_str(&current.1);
        hasher.update(to_hash.as_bytes());
        format!("{:x}", hasher.finalize())
            .chars()
            .take(4)
            .collect::<String>()
    };

    if current.0 .1 != 0 && hash.chars().next().unwrap() >= 'b' {
        next.push((
            (current.0 .0, current.0 .1 - 1),
            current.1.to_string() + "U",
        ));
    }

    if current.0 .1 != 3 && hash.chars().nth(1).unwrap() >= 'b' {
        next.push((
            (current.0 .0, current.0 .1 + 1),
            current.1.to_string() + "D",
        ));
    }
    if current.0 .0 != 0 && hash.chars().nth(2).unwrap() >= 'b' {
        next.push((
            (current.0 .0 - 1, current.0 .1),
            current.1.to_string() + "L",
        ));
    }
    if current.0 .0 != 3 && hash.chars().nth(3).unwrap() >= 'b' {
        next.push((
            (current.0 .0 + 1, current.0 .1),
            current.1.to_string() + "R",
        ));
    }

    next
}

fn part1(input: &str) -> String {
    let passcode = input.trim();

    let mut queue = VecDeque::from_iter([((0, 0), String::new())]);

    while let Some(current) = queue.pop_front() {
        for next in get_next(current, passcode) {
            if next.0 == (3, 3) {
                return next.1;
            }
            queue.push_back(next);
        }
    }

    String::new()
}

fn part2(input: &str) -> usize {
    let passcode = input.trim();

    let mut queue = VecDeque::from_iter([(0, (0, 0), String::new())]);
    let mut max = 0;

    while let Some(current) = queue.pop_front() {
        for next in get_next((current.1, current.2), passcode) {
            if next.0 == (3, 3) {
                if current.0 + 1 > max {
                    max = current.0 + 1;
                }
                continue;
            }
            queue.push_back((current.0 + 1, next.0, next.1));
        }
    }

    max
}
