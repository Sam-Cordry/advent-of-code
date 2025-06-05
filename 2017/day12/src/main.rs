use std::{
    collections::{HashMap, HashSet, VecDeque},
    iter::FromIterator,
};

fn main() {
    let input = include_str!("../input");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn can_reach_zero(
    start: usize,
    graph: &HashMap<usize, Vec<usize>>,
    prev: &HashMap<usize, bool>,
) -> bool {
    if start == 0 || prev[&start] {
        return true;
    }

    let mut queue: VecDeque<usize> = VecDeque::from_iter([start]);
    let mut seen: HashSet<usize> = HashSet::from_iter([start]);

    while let Some(current) = queue.pop_front() {
        for n in graph[&current].clone() {
            if n == 0 || prev[&n] {
                return true;
            } else if !seen.contains(&n) {
                seen.insert(n);
                queue.push_back(n);
            }
        }
    }

    false
}

fn part1(input: &str) -> usize {
    let mut graph: HashMap<usize, Vec<usize>> =
        HashMap::from_iter((0..input.lines().count()).map(|n| (n, vec![])));

    for (current, mut next) in input.lines().map(|l| {
        let split = l.split("<->").collect::<Vec<&str>>();
        (
            split[0].trim().parse::<usize>().unwrap(),
            split[1]
                .split(",")
                .map(|n| n.trim().parse::<usize>().unwrap())
                .collect::<Vec<usize>>(),
        )
    }) {
        for n in next.iter() {
            graph.entry(*n).and_modify(|v| v.push(current));
        }
        graph.entry(current).and_modify(|v| v.append(&mut next));
    }

    let mut prev: HashMap<usize, bool> =
        HashMap::from_iter((0..input.lines().count()).map(|l| (l, false)));
    let mut result = 0;
    for start in graph.keys() {
        if can_reach_zero(*start, &graph, &prev) {
            result += 1;
            prev.insert(*start, true);
        }
    }

    result
}

fn part2(input: &str) -> usize {
    let mut graph: HashMap<usize, Vec<usize>> =
        HashMap::from_iter((0..input.lines().count()).map(|n| (n, vec![])));

    for (current, mut next) in input.lines().map(|l| {
        let split = l.split("<->").collect::<Vec<&str>>();
        (
            split[0].trim().parse::<usize>().unwrap(),
            split[1]
                .split(",")
                .map(|n| n.trim().parse::<usize>().unwrap())
                .collect::<Vec<usize>>(),
        )
    }) {
        for n in next.iter() {
            graph.entry(*n).and_modify(|v| v.push(current));
        }
        graph.entry(current).and_modify(|v| v.append(&mut next));
    }

    let mut visited = vec![false; input.lines().count()];
    let mut result = 0;
    for c in graph.keys() {
        if !visited[*c] {
            result += 1;

            let mut queue = VecDeque::from_iter([*c]);
            visited[*c] = true;

            while let Some(current) = queue.pop_front() {
                for n in graph[&current].clone() {
                    if !visited[n] {
                        queue.push_back(n);
                        visited[n] = true;
                    }
                }
            }
        }
    }

    result
}
