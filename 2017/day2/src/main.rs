fn main() {
    let input = include_str!("../input");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|l| {
            let values = l
                .split_whitespace()
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            values.iter().max().unwrap() - values.iter().min().unwrap()
        })
        .sum()
}

fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|l| {
            let mut values = l
                .split_whitespace()
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            values.sort();

            for i in 1..values.len() {
                for j in 0..i {
                    if values[i] % values[j] == 0 {
                        return values[i] / values[j];
                    }
                }
            }

            0
        })
        .sum()
}
