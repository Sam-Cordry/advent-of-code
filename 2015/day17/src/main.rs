use itertools::Itertools;

fn main() {
    let input = include_str!("../input");

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|l| l.parse::<usize>().unwrap())
        .powerset()
        .filter(|c| c.iter().sum::<usize>() == 150)
        .count()
}

fn part2(input: &str) -> usize {
    let min = input
        .lines()
        .map(|l| l.parse::<usize>().unwrap())
        .powerset()
        .filter(|c| c.iter().sum::<usize>() == 150)
        .map(|c| c.len())
        .min()
        .unwrap();

    input
        .lines()
        .map(|l| l.parse::<usize>().unwrap())
        .powerset()
        .filter(|c| c.iter().sum::<usize>() == 150 && c.len() == min)
        .count()
}
