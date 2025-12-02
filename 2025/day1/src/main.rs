fn main() {
    let input = include_str!("../input");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let mut result = 0;
    input
        .lines()
        .map(|l| {
            if let Some(n) = l.strip_prefix('L') {
                (-n.parse::<isize>().unwrap()).rem_euclid(100)
            } else {
                l[1..].parse::<isize>().unwrap()
            }
        })
        .fold(50, |acc, n| {
            if (acc + n) % 100 == 0 {
                result += 1;
            }
            acc + n
        });
    result
}

fn part2(input: &str) -> usize {
    let mut result = 0;
    input
        .lines()
        .map(|l| {
            if let Some(n) = l.strip_prefix('L') {
                -n.parse::<isize>().unwrap()
            } else {
                l[1..].parse::<isize>().unwrap()
            }
        })
        .fold(50, |acc, n| {
            result += (n / 100).unsigned_abs();
            if (acc + (n % 100)) >= 100 || ((acc + (n % 100)) <= 0 && acc != 0) {
                result += 1;
            }
            (acc + n).rem_euclid(100)
        });
    result
}
