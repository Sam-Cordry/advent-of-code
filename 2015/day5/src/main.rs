fn main() {
    let input = include_str!("../input");

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .filter(|l| {
            l.chars()
                .filter(|c| *c == 'a' || *c == 'e' || *c == 'i' || *c == 'o' || *c == 'u')
                .count()
                >= 3
                && l.chars()
                    .collect::<Vec<_>>()
                    .windows(2)
                    .any(|p| p[0] == p[1])
                && l.chars().collect::<Vec<_>>().windows(2).all(|p| {
                    (p[0] != 'a' || p[1] != 'b')
                        && (p[0] != 'c' || p[1] != 'd')
                        && (p[0] != 'p' || p[1] != 'q')
                        && (p[0] != 'x' || p[1] != 'y')
                })
        })
        .count()
}

fn part2(input: &str) -> usize {
    input
        .lines()
        .filter(|l| {
            for pair in l.chars().enumerate().collect::<Vec<_>>().windows(2) {
                for test in l.chars().enumerate().collect::<Vec<_>>().windows(2) {
                    if pair[0].1 == test[0].1
                        && pair[1].1 == test[1].1
                        && pair[0].0 != test[0].0
                        && pair[1].0 != test[0].0
                        && pair[0].0 != test[1].0
                    {
                        return l
                            .chars()
                            .collect::<Vec<_>>()
                            .windows(3)
                            .any(|t| t[0] == t[2]);
                    }
                }
            }
            false
        })
        .count()
}
