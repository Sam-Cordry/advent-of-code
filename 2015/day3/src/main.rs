use std::collections::HashSet;

fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

fn part1() -> usize {
    let mut current: (i32, i32) = (0, 0);
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert((0, 0));

    for coord in std::fs::read_to_string("input")
        .unwrap()
        .trim()
        .chars()
        .map(|c| {
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
        })
    {
        visited.insert(coord);
    }

    visited.len()
}

fn part2() -> usize {
    let mut current_a: (i32, i32) = (0, 0);
    let mut current_b: (i32, i32) = (0, 0);

    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert((0, 0));

    for (idx, c) in std::fs::read_to_string("input")
        .unwrap()
        .trim()
        .chars()
        .enumerate()
    {
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
