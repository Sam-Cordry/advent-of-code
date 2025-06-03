use std::collections::HashSet;

fn main() {
    let input = include_str!("../input");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|l| l.split_whitespace().collect::<Vec<&str>>())
        .filter(|l| HashSet::<&&str>::from_iter(l.iter()).len() == l.len())
        .count()
}

fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|s| {
                    let mut sorted = s.chars().collect::<Vec<char>>();
                    sorted.sort();

                    sorted.iter().collect::<String>()
                })
                .collect::<Vec<String>>()
        })
        .filter(|l| HashSet::<&str>::from_iter(l.iter().map(|s| s.as_str())).len() == l.len())
        .count()
}
