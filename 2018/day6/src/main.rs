use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("../input");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let origins: Vec<(usize, usize)> = input
        .lines()
        .map(|l| {
            let mut t = l.split(',');
            (
                t.next().unwrap().parse().unwrap(),
                t.next().unwrap().trim().parse().unwrap(),
            )
        })
        .collect();
    let mut grid: Vec<Vec<Vec<usize>>> =
        vec![
            vec![vec![]; origins.iter().map(|c| c.1).max().unwrap()];
            origins.iter().map(|c| c.0).max().unwrap()
        ];

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            let distances: Vec<usize> = origins
                .iter()
                .map(|c| c.0.abs_diff(i) + c.1.abs_diff(j))
                .collect();
            let min = distances.iter().min().unwrap();

            grid[i][j] = distances
                .iter()
                .enumerate()
                .filter_map(|(i, d)| if d == min { Some(i) } else { None })
                .collect();
        }
    }

    let mut infinite: HashSet<usize> = HashSet::new();

    for i in 0..grid.len() {
        for e in grid[i][0].iter() {
            infinite.insert(*e);
        }
        for e in grid[i][grid[0].len() - 1].iter() {
            infinite.insert(*e);
        }
    }
    for i in 0..grid[0].len() {
        for e in grid[0][i].iter() {
            infinite.insert(*e);
        }
        for e in grid[grid.len() - 1][i].iter() {
            infinite.insert(*e);
        }
    }

    let mut freq: HashMap<usize, usize> = HashMap::new();

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j].len() == 1 {
                freq.entry(*grid[i][j].first().unwrap())
                    .and_modify(|v| *v += 1)
                    .or_insert(1);
            }
        }
    }

    *freq
        .iter()
        .filter_map(|e| {
            if !infinite.contains(e.0) {
                Some(e.1)
            } else {
                None
            }
        })
        .max()
        .unwrap()
}

fn part2(input: &str) -> usize {
    let origins: Vec<(usize, usize)> = input
        .lines()
        .map(|l| {
            let mut t = l.split(',');
            (
                t.next().unwrap().parse().unwrap(),
                t.next().unwrap().trim().parse().unwrap(),
            )
        })
        .collect();
    let mut result = 0;

    for i in 0..origins.iter().map(|c| c.0).max().unwrap() {
        for j in 0..origins.iter().map(|c| c.1).max().unwrap() {
            if origins
                .iter()
                .map(|c| c.0.abs_diff(i) + c.1.abs_diff(j))
                .sum::<usize>()
                < 10000
            {
                result += 1;
            }
        }
    }

    result
}
