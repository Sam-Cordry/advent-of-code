use itertools::Itertools;
use std::collections::HashMap;

fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

fn part1() -> usize {
    let mut nodes: HashMap<&str, HashMap<&str, usize>> = HashMap::new();

    let binding = std::fs::read_to_string("input").unwrap();
    for l in binding.lines().map(|l| l.trim()) {
        let dist = l.split_whitespace().collect::<Vec<&str>>();

        if !nodes.contains_key(dist[0]) {
            nodes.insert(dist[0], HashMap::new());
        }
        if !nodes.contains_key(dist[2]) {
            nodes.insert(dist[2], HashMap::new());
        }

        nodes
            .get_mut(dist[0])
            .unwrap()
            .insert(dist[2], dist[4].parse().unwrap());
        nodes
            .get_mut(dist[2])
            .unwrap()
            .insert(dist[0], dist[4].parse().unwrap());
    }

    let mut min: usize = nodes
        .iter()
        .map(|e| e.1.iter().map(|n| n.1).sum::<usize>())
        .sum();

    for p in nodes.keys().permutations(nodes.len()) {
        let mut total: usize = 0;
        for i in 0..p.len() - 1 {
            total += nodes.get(p[i]).unwrap().get(p[i + 1]).unwrap();
        }
        if min > total {
            min = total;
        }
    }

    min
}

fn part2() -> usize {
    let mut nodes: HashMap<&str, HashMap<&str, usize>> = HashMap::new();

    let binding = std::fs::read_to_string("input").unwrap();
    for l in binding.lines().map(|l| l.trim()) {
        let dist = l.split_whitespace().collect::<Vec<&str>>();

        if !nodes.contains_key(dist[0]) {
            nodes.insert(dist[0], HashMap::new());
        }
        if !nodes.contains_key(dist[2]) {
            nodes.insert(dist[2], HashMap::new());
        }

        nodes
            .get_mut(dist[0])
            .unwrap()
            .insert(dist[2], dist[4].parse().unwrap());
        nodes
            .get_mut(dist[2])
            .unwrap()
            .insert(dist[0], dist[4].parse().unwrap());
    }

    let mut max: usize = 0;

    for p in nodes.keys().permutations(nodes.len()) {
        let mut total: usize = 0;
        for i in 0..p.len() - 1 {
            total += nodes.get(p[i]).unwrap().get(p[i + 1]).unwrap();
        }
        if max < total {
            max = total;
        }
    }

    max
}
