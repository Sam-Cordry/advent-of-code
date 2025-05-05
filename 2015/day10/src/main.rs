use itertools::Itertools;

fn main() {
    let input = include_str!("../input");

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn next(line: &str) -> String {
    let mut next: String = String::new();

    for (key, chunk) in &line.chars().chunk_by(|c| *c) {
        next += &chunk.count().to_string();
        next += &key.to_string();
    }

    next
}

fn part1(input: &str) -> usize {
    let mut current: String = input.trim().to_string();

    for _ in 0..40 {
        current = next(&current);
    }

    current.len()
}

fn part2(input: &str) -> usize {
    let mut current: String = input.trim().to_string();

    for _ in 0..50 {
        current = next(&current);
    }

    current.len()
}
