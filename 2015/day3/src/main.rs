use std::collections::HashSet;

fn main() {
    let input = include_str!("../input");

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let mut current: (i32, i32) = (0, 0);
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert((0, 0));

    for coord in input.chars().map(|c| {
        if c == '>' {
            current.0 += 1;
        } else if c == '<' {
            current.0 -= 1;
        } else if c == '^' {
            current.1 += 1;
        } else {
            current.1 -= 1;
        }
        current
    }) {
        visited.insert(coord);
    }

    visited.len()
}

fn part2(input: &str) -> usize {
    let mut current_a: (i32, i32) = (0, 0);
    let mut current_b: (i32, i32) = (0, 0);

    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert((0, 0));

    for (idx, c) in input.chars().enumerate() {
        if idx % 2 == 0 {
            if c == '>' {
                current_a.0 += 1;
            } else if c == '<' {
                current_a.0 -= 1;
            } else if c == '^' {
                current_a.1 += 1;
            } else {
                current_a.1 -= 1;
            }
            visited.insert(current_a);
        } else {
            if c == '>' {
                current_b.0 += 1;
            } else if c == '<' {
                current_b.0 -= 1;
            } else if c == '^' {
                current_b.1 += 1;
            } else {
                current_b.1 -= 1;
            }
            visited.insert(current_b);
        }
    }

    visited.len()
}
