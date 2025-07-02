use std::collections::HashMap;

fn main() {
    let input = include_str!("../input");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn solve(input: &str, n: usize) -> isize {
    let mut plants = vec!['.'; 3];
    plants.extend(
        input
            .lines()
            .next()
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .trim()
            .chars(),
    );
    plants.extend(['.'; 3]);

    let rules: HashMap<&str, char> = HashMap::from_iter(input.lines().skip(2).map(|l| {
        let split = l.split_whitespace().collect::<Vec<&str>>();
        (split[0], split[2].chars().next().unwrap())
    }));

    let mut diffs: HashMap<isize, usize> = HashMap::new();
    let mut prev = 0;

    for g in 0..n {
        let mut temp = vec!['.'; 3];

        for i in 2..(plants.len() - 2) {
            if let Some(c) = rules.get(
                &plants
                    .iter()
                    .skip(i - 2)
                    .take(5)
                    .collect::<String>()
                    .as_str(),
            ) {
                temp.push(*c);
            } else {
                temp.push(plants[i]);
            }
        }

        temp.extend(['.'; 3]);
        plants = temp;

        let current = plants
            .iter()
            .enumerate()
            .filter_map(|(i, p)| {
                if *p == '#' {
                    Some(i as isize - g as isize - 4)
                } else {
                    None
                }
            })
            .sum::<isize>();
        let d = diffs.entry(current - prev).or_insert(0);

        if *d > 10 {
            return (n - (g + 1)) as isize * (current - prev) + current;
        } else {
            *d += 1;
        }

        prev = current;
    }

    prev
}

fn part1(input: &str) -> isize {
    solve(input, 20)
}

fn part2(input: &str) -> isize {
    solve(input, 50000000000)
}
