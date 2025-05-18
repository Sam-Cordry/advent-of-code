fn main() {
    let input = include_str!("../input");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|l| {
            let mut t = l
                .split_whitespace()
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            t.sort();
            t
        })
        .filter(|t| t[0] + t[1] > t[2])
        .count()
}

fn part2(input: &str) -> usize {
    input
        .lines()
        .flat_map(|l| {
            l.split_whitespace()
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<usize>>()
        .chunks(9)
        .flat_map(|p| {
            vec![
                vec![p[0], p[3], p[6]],
                vec![p[1], p[4], p[7]],
                vec![p[2], p[5], p[8]],
            ]
        })
        .map(|t| {
            let mut new = t;
            new.sort();
            new
        })
        .filter(|t| t[0] + t[1] > t[2])
        .count()
}
