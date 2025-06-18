use std::collections::HashSet;

fn main() {
    let input = include_str!("../input");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> i32 {
    input
        .lines()
        .map(|l| {
            if let Some(s) = l.strip_prefix("+") {
                s.parse::<i32>().unwrap()
            } else {
                l.parse::<i32>().unwrap()
            }
        })
        .sum()
}

fn part2(input: &str) -> i32 {
    let changes: Vec<i32> = input
        .lines()
        .map(|l| {
            if let Some(s) = l.strip_prefix("+") {
                s.parse::<i32>().unwrap()
            } else {
                l.parse::<i32>().unwrap()
            }
        })
        .collect();

    let mut seen: HashSet<i32> = HashSet::from_iter([0]);
    let mut freq: i32 = 0;
    let mut i = 0;

    loop {
        freq += changes[i % changes.len()];

        if seen.contains(&freq) {
            return freq;
        }
        seen.insert(freq);

        i += 1;
    }
}
