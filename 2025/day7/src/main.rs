use std::collections::HashSet;

fn main() {
    let input = include_str!("../input");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let mut current: HashSet<usize> =
        HashSet::from([input.lines().next().unwrap().find('S').unwrap()]);
    let mut count = 0;

    for l in input
        .lines()
        .skip(1)
        .map(|l| l.trim().chars().collect::<Vec<char>>())
    {
        let mut next: HashSet<usize> = current.clone();

        for (i, c) in l.iter().enumerate() {
            if current.contains(&i) && *c == '^' {
                next.remove(&i);
                next.insert(i - 1);
                next.insert(i + 1);
                count += 1;
            }
        }

        current = next;
    }

    count
}

fn part2(input: &str) -> usize {
    let length = input.lines().next().unwrap().len();
    let mut current: Vec<usize> = vec![0; length];
    current[input
        .lines()
        .next()
        .unwrap()
        .chars()
        .position(|c| c == 'S')
        .unwrap()] = 1;

    for l in input
        .lines()
        .skip(1)
        .map(|l| l.trim().chars().collect::<Vec<char>>())
    {
        let mut new: Vec<usize> = vec![0; length];

        for (i, c) in l.iter().enumerate() {
            if current[i] == 0 {
                continue;
            }

            if *c == '.' {
                new[i] += current[i];
            } else if *c == '^' {
                if i > 0 {
                    new[i - 1] += current[i];
                }
                if i < length {
                    new[i + 1] += current[i];
                }
            }
        }

        current = new;
    }

    current.iter().sum()
}
