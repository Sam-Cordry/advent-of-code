use std::collections::{HashSet, VecDeque};

fn main() {
    let input = include_str!("../input");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn is_open(current: (usize, usize), offset: usize) -> bool {
    format!(
        "{:b}",
        current.0 * current.0
            + 3 * current.0
            + 2 * current.0 * current.1
            + current.1
            + current.1 * current.1
            + offset
    )
    .chars()
    .filter(|c| *c == '1')
    .count()
        % 2
        == 0
}

fn get_neighbors(current: (usize, usize), offset: usize) -> Vec<(usize, usize)> {
    let mut result: HashSet<(usize, usize)> = HashSet::new();

    for d in [(2, 1), (0, 1), (1, 2), (1, 0)] {
        let temp = (
            (current.0 + d.0).saturating_sub(1),
            (current.1 + d.1).saturating_sub(1),
        );
        if is_open(temp, offset) {
            result.insert(temp);
        }
    }

    Vec::from_iter(result)
}

fn part1(input: &str) -> usize {
    let offset = input.trim().parse::<usize>().unwrap();

    let mut queue: VecDeque<(usize, (usize, usize))> = VecDeque::new();
    queue.push_back((0, (1, 1)));

    let mut seen: HashSet<(usize, usize)> = HashSet::new();
    seen.insert((1, 1));

    while let Some((steps, current)) = queue.pop_front() {
        for n in get_neighbors(current, offset) {
            if n == (31, 39) {
                return steps + 1;
            } else if !seen.contains(&n) {
                seen.insert(n);
                queue.push_back((steps + 1, n));
            }
        }
    }

    0
}

fn part2(input: &str) -> usize {
    let offset = input.trim().parse::<usize>().unwrap();

    let mut queue: VecDeque<(usize, (usize, usize))> = VecDeque::new();
    queue.push_back((0, (1, 1)));

    let mut seen: HashSet<(usize, usize)> = HashSet::new();
    seen.insert((1, 1));

    while let Some((steps, current)) = queue.pop_front() {
        if steps == 50 {
            break;
        }

        for n in get_neighbors(current, offset) {
            if !seen.contains(&n) {
                seen.insert(n);
                queue.push_back((steps + 1, n));
            }
        }
    }

    seen.len()
}
