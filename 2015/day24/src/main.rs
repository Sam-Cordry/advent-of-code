use itertools::Itertools;

fn main() {
    let input = include_str!("../input");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let nums = input
        .lines()
        .map(|l| l.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let target = nums.iter().sum::<usize>() / 3;

    let mut found = false;
    let mut count = 1;
    loop {
        for p in nums.iter().permutations(count) {
            if p.into_iter().sum::<usize>() == target {
                found = true;
                break;
            }
        }

        if found {
            break;
        }

        count += 1;
    }

    let mut min = usize::MAX;
    for p in nums.into_iter().permutations(count) {
        let sum = p.iter().sum::<usize>();
        let product = p.iter().product::<usize>();

        if sum == target && product < min {
            min = product;
        }
    }

    min
}

fn part2(input: &str) -> usize {
    let nums = input
        .lines()
        .map(|l| l.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let target = nums.iter().sum::<usize>() / 4;

    let mut found = false;
    let mut count = 1;
    loop {
        for p in nums.iter().permutations(count) {
            if p.into_iter().sum::<usize>() == target {
                found = true;
                break;
            }
        }

        if found {
            break;
        }

        count += 1;
    }

    let mut min = usize::MAX;
    for p in nums.into_iter().permutations(count) {
        let sum = p.iter().sum::<usize>();
        let product = p.iter().product::<usize>();

        if sum == target && product < min {
            min = product;
        }
    }

    min
}
