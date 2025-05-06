use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("../input");

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn get_next(current: &str, replacements: &HashMap<&str, Vec<&str>>) -> HashSet<String> {
    let mut found: HashSet<String> = HashSet::new();

    for r in replacements.keys() {
        for f in current.match_indices(r) {
            for s in replacements.get(r).unwrap() {
                let mut result = String::new();

                result.push_str(&current[..f.0]);
                result.push_str(s);
                result.push_str(&current[f.0 + r.len()..]);

                found.insert(result);
            }
        }
    }

    found
}

fn part1(input: &str) -> usize {
    let original = input.lines().last().unwrap();
    let mut replacements: HashMap<&str, Vec<&str>> = HashMap::new();

    for line in input
        .lines()
        .take_while(|l| l.split_whitespace().collect::<Vec<&str>>().len() == 3)
        .map(|l| l.split_whitespace().collect::<Vec<&str>>())
    {
        if !replacements.contains_key(line[0]) {
            replacements.insert(line[0], vec![]);
        }

        replacements.get_mut(line[0]).unwrap().push(line[2]);
    }

    get_next(original, &replacements).len()
}

fn part2(input: &str) -> usize {
    let mut current = input.lines().last().unwrap().to_string();
    let mut replacements: HashMap<&str, &str> = HashMap::new();

    for line in input
        .lines()
        .take_while(|l| l.split_whitespace().collect::<Vec<&str>>().len() == 3)
        .map(|l| l.split_whitespace().collect::<Vec<&str>>())
    {
        replacements.insert(line[2], line[0]);
    }

    let mut steps = 0;
    while current != "e" {
        for r in replacements.keys() {
            if let Some(idx) = current.find(r) {
                let mut new = String::new();
                new.push_str(&current[..idx]);
                new.push_str(replacements.get(r).unwrap());
                new.push_str(&current[idx + r.len()..]);
                current = new;
                steps += 1;
            }
        }
    }

    steps
}
