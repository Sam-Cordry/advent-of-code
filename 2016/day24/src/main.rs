use std::collections::{HashSet, VecDeque};

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
struct State {
    position: (usize, usize),
    visited: Vec<bool>,
}

fn get_next(current: &State, grid: &[Vec<char>]) -> Vec<State> {
    let mut result = vec![];

    for (x, y) in [(0, 1), (2, 1), (1, 0), (1, 2)] {
        if current.position.0 + x - 1 < grid[0].len()
            && current.position.1 + y - 1 < grid.len()
            && grid[current.position.1 + y - 1][current.position.0 + x - 1] != '#'
        {
            let mut new = State {
                position: (current.position.0 + x - 1, current.position.1 + y - 1),
                visited: current.visited.clone(),
            };

            if let Ok(n) = grid[current.position.1 + y - 1][current.position.0 + x - 1]
                .to_string()
                .parse::<usize>()
            {
                new.visited[n] = true;
            }

            result.push(new);
        }
    }

    result
}

fn main() {
    let input = include_str!("../input");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let mut grid: Vec<Vec<char>> =
        vec![vec!['*'; input.lines().next().unwrap().len()]; input.lines().count()];
    let mut max = 0;
    let mut position = (0, 0);

    for (i, row) in input.lines().map(|l| l.trim()).enumerate() {
        for (j, c) in row.chars().enumerate() {
            grid[i][j] = c;

            if let Ok(n) = c.to_string().parse::<usize>() {
                if n == 0 {
                    position = (j, i);
                } else if n > max {
                    max = n;
                }
            }
        }
    }

    let mut visited = vec![false; max + 1];
    visited[0] = true;

    let initial = State { position, visited };

    let mut queue = VecDeque::from_iter([(0, initial.clone())]);
    let mut seen: HashSet<State> = HashSet::from_iter([initial]);

    while let Some((steps, current)) = queue.pop_front() {
        for n in get_next(&current, &grid) {
            if !n.visited.iter().any(|&v| !v) {
                return steps + 1;
            } else if !seen.contains(&n) {
                seen.insert(n.clone());
                queue.push_back((steps + 1, n));
            }
        }
    }

    0
}

fn part2(input: &str) -> usize {
    let mut grid: Vec<Vec<char>> =
        vec![vec!['*'; input.lines().next().unwrap().len()]; input.lines().count()];
    let mut max = 0;
    let mut position = (0, 0);

    for (i, row) in input.lines().map(|l| l.trim()).enumerate() {
        for (j, c) in row.chars().enumerate() {
            grid[i][j] = c;

            if let Ok(n) = c.to_string().parse::<usize>() {
                if n == 0 {
                    position = (j, i);
                } else if n > max {
                    max = n;
                }
            }
        }
    }

    let mut visited = vec![false; max + 1];
    visited[0] = true;

    let initial = State { position, visited };

    let mut queue = VecDeque::from_iter([(0, initial.clone())]);
    let mut seen: HashSet<State> = HashSet::from_iter([initial]);

    while let Some((steps, current)) = queue.pop_front() {
        for n in get_next(&current, &grid) {
            if !n.visited.iter().any(|&v| !v) && n.position == position {
                return steps + 1;
            } else if !seen.contains(&n) {
                seen.insert(n.clone());
                queue.push_back((steps + 1, n));
            }
        }
    }

    0
}
