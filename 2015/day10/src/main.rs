use itertools::Itertools;

fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

fn next(line: &str) -> String {
    let mut next: String = String::new();

    for (key, chunk) in &line.chars().chunk_by(|c| *c) {
        next += &chunk.count().to_string();
        next += &key.to_string();
    }

    next
}

fn part1() -> usize {
    let mut current: String = std::fs::read_to_string("input").unwrap().trim().to_string();

    for _ in 0..40 {
        current = next(&current);
    }

    current.len()
}

fn part2() -> usize {
    let mut current: String = std::fs::read_to_string("input").unwrap().trim().to_string();

    for _ in 0..50 {
        current = next(&current);
    }

    current.len()
}
