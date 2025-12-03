use std::collections::HashSet;

fn main() {
    let input = include_str!("../input");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let mut ranges: Vec<(usize, usize)> = input
        .split(",")
        .map(|r| {
            let split = r.split("-").collect::<Vec<&str>>();
            (
                split[0].trim().parse::<usize>().unwrap(),
                split[1].trim().parse::<usize>().unwrap(),
            )
        })
        .collect();
    ranges.sort_by(|a, b| a.0.cmp(&b.0));
    let mut result = 0;

    for i in (1..)
        .take_while(|n| n.to_string().len() * 2 <= ranges[ranges.len() - 1].1.to_string().len())
    {
        let test = i * 10usize.pow(i.to_string().len() as u32) + i;
        if ranges.iter().any(|r| r.0 <= test && test <= r.1) {
            result += test;
        }
    }

    result
}

fn part2(input: &str) -> usize {
    let mut ranges: Vec<(usize, usize)> = input
        .split(",")
        .map(|r| {
            let split = r.split("-").collect::<Vec<&str>>();
            (
                split[0].trim().parse::<usize>().unwrap(),
                split[1].trim().parse::<usize>().unwrap(),
            )
        })
        .collect();
    ranges.sort_by(|a, b| a.0.cmp(&b.0));
    let mut result: HashSet<usize> = HashSet::new();

    for i in (1..)
        .take_while(|n| n.to_string().len() * 2 <= ranges[ranges.len() - 1].1.to_string().len())
    {
        for j in (2..)
            .take_while(|n| i.to_string().len() * n <= ranges[ranges.len() - 1].1.to_string().len())
        {
            let mut test = 0;
            for k in 0..j {
                test += i * 10usize.pow((i.to_string().len() * k) as u32);
            }
            if ranges.iter().any(|r| r.0 <= test && test <= r.1) {
                result.insert(test);
            }
        }
    }

    result.iter().sum()
}
