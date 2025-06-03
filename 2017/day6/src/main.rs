use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("../input");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let mut banks = input
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let mut seen: HashSet<Vec<usize>> = HashSet::new();
    let mut count = 0;

    while !seen.contains(&banks) {
        seen.insert(banks.clone());

        let max = *banks.iter().max().unwrap();
        let idx = banks.iter().position(|&b| b == max).unwrap();
        banks[idx] = 0;

        let len = banks.len();
        for i in (idx + 1)..(idx + 1 + max) {
            banks[i % len] += 1;
        }

        count += 1;
    }

    count
}

fn part2(input: &str) -> usize {
    let mut banks = input
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let mut seen: HashSet<Vec<usize>> = HashSet::new();
    let mut known: HashMap<Vec<usize>, usize> = HashMap::new();
    let mut count = 0;

    while !seen.contains(&banks) {
        seen.insert(banks.clone());
        known.insert(banks.clone(), count);

        let max = *banks.iter().max().unwrap();
        let idx = banks.iter().position(|&b| b == max).unwrap();
        banks[idx] = 0;

        let len = banks.len();
        for i in (idx + 1)..(idx + 1 + max) {
            banks[i % len] += 1;
        }

        count += 1;
    }

    count - known.get(&banks).unwrap()
}
