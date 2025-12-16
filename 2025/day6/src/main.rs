use itertools::izip;

fn main() {
    let input = include_str!("../input");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let parsed: Vec<Vec<&str>> = input
        .lines()
        .map(|l| l.split_whitespace().collect())
        .collect();
    izip!(
        parsed[0].clone(),
        parsed[1].clone(),
        parsed[2].clone(),
        parsed[3].clone(),
        parsed[4].clone()
    )
    .map(|(a, b, c, d, op)| {
        if op == "+" {
            a.parse::<usize>().unwrap()
                + b.parse::<usize>().unwrap()
                + c.parse::<usize>().unwrap()
                + d.parse::<usize>().unwrap()
        } else {
            a.parse::<usize>().unwrap()
                * b.parse::<usize>().unwrap()
                * c.parse::<usize>().unwrap()
                * d.parse::<usize>().unwrap()
        }
    })
    .sum()
}

fn part2(input: &str) -> usize {
    let lines: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut problems: Vec<(Vec<usize>, char)> = vec![];
    let mut start = 0;
    let mut i = 0;

    while i < lines[0].len() {
        i = start;
        loop {
            if lines.iter().all(|l| i >= l.len() || l[i] == ' ') {
                break;
            }
            i += 1;
        }

        problems.push((vec![], lines[4][start]));
        let last = problems.len() - 1;

        for j in start..i {
            let mut current = String::new();
            for l in lines.iter().take(4) {
                current.push(l[j]);
            }
            problems[last].0.push(current.trim().parse().unwrap());
        }

        start = i + 1;
    }

    problems
        .iter()
        .map(|p| {
            if p.1 == '+' {
                p.0.iter().sum::<usize>()
            } else {
                p.0.iter().product()
            }
        })
        .sum()
}
