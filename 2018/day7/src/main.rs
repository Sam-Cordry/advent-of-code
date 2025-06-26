use std::collections::{HashMap, HashSet};

#[derive(PartialEq, Clone, Debug)]
enum WorkerState {
    Ready,
    Working(char, usize),
}

fn main() {
    let input = include_str!("../input");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> String {
    let mut prereqs: HashMap<char, Vec<char>> = HashMap::new();

    for (prereq, current) in input.lines().map(|l| {
        let split = l.split_whitespace().collect::<Vec<&str>>();
        (
            split[1].chars().next().unwrap(),
            split[7].chars().next().unwrap(),
        )
    }) {
        prereqs
            .entry(current)
            .and_modify(|v| v.push(prereq))
            .or_insert(vec![prereq]);

        prereqs.entry(prereq).or_default();
    }

    let mut result = String::new();

    while result.len() < prereqs.len() {
        let mut ready: Vec<char> = prereqs
            .iter()
            .filter_map(|e| {
                if !result.contains(*e.0) && e.1.iter().all(|c| result.contains(*c)) {
                    Some(*e.0)
                } else {
                    None
                }
            })
            .collect();
        ready.sort();

        result.push(*ready.first().unwrap());
    }

    result
}

fn part2(input: &str) -> usize {
    let mut prereqs: HashMap<char, Vec<char>> = HashMap::new();

    for (prereq, current) in input.lines().map(|l| {
        let split = l.split_whitespace().collect::<Vec<&str>>();
        (
            split[1].chars().next().unwrap(),
            split[7].chars().next().unwrap(),
        )
    }) {
        prereqs
            .entry(current)
            .and_modify(|v| v.push(prereq))
            .or_insert(vec![prereq]);

        prereqs.entry(prereq).or_default();
    }

    let mut done: HashSet<char> = HashSet::new();
    let mut workers = vec![WorkerState::Ready; 5];
    let mut time = 0;

    while done.len() < prereqs.len() {
        for w in workers.iter_mut() {
            if let WorkerState::Working(c, t) = *w {
                if t == time {
                    *w = WorkerState::Ready;
                    done.insert(c);
                }
            }
        }

        let mut ready: Vec<char> = prereqs
            .iter()
            .filter_map(|e| {
                if !done.contains(e.0)
                    && e.1.iter().all(|c| done.contains(c))
                    && !workers.iter().any(|w| {
                        if let WorkerState::Working(c, _) = *w {
                            c == *e.0
                        } else {
                            false
                        }
                    })
                {
                    Some(*e.0)
                } else {
                    None
                }
            })
            .collect();
        ready.sort();

        for w in workers.iter_mut() {
            if ready.is_empty() {
                break;
            }

            if *w == WorkerState::Ready {
                let c = ready.pop().unwrap();
                *w = WorkerState::Working(c, time + 60 + (c as u8 - 64) as usize);
            }
        }

        time += 1;
    }

    time - 1
}
