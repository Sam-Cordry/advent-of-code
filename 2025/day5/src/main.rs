fn main() {
    let input = include_str!("../input");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let ranges: Vec<(usize, usize)> = input
        .lines()
        .take_while(|l| !l.trim().is_empty())
        .map(|l| {
            let split: Vec<usize> = l
                .trim()
                .split('-')
                .map(|n| n.parse::<usize>().unwrap())
                .collect();
            (split[0], split[1])
        })
        .collect();
    let ingredients: Vec<usize> = input
        .lines()
        .skip_while(|l| !l.trim().is_empty())
        .skip(1)
        .map(|l| l.trim().parse::<usize>().unwrap())
        .collect();

    ingredients
        .iter()
        .filter(|&&i| ranges.iter().any(|r| r.0 <= i && i <= r.1))
        .count()
}

fn part2(input: &str) -> usize {
    let mut ranges: Vec<(usize, usize)> = input
        .lines()
        .take_while(|l| !l.trim().is_empty())
        .map(|l| {
            let split: Vec<usize> = l
                .trim()
                .split('-')
                .map(|n| n.parse::<usize>().unwrap())
                .collect();
            (split[0], split[1])
        })
        .collect();
    ranges.sort_by(|a, b| a.0.cmp(&b.0));

    ranges
        .iter()
        .fold(vec![], |mut acc: Vec<(usize, usize)>, r| {
            if acc.is_empty() {
                acc.push(*r);
            } else if acc.last().unwrap().1 >= r.0 {
                let temp = acc.len() - 1;
                acc[temp].1 = acc[temp].1.max(r.1);
            } else {
                acc.push(*r);
            }

            acc
        })
        .iter()
        .map(|r| r.1 - r.0 + 1)
        .sum()
}
