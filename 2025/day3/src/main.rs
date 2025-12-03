use std::cmp::Ordering;

fn main() {
    let input = include_str!("../input");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|l| {
            let chars: Vec<usize> = l
                .trim()
                .chars()
                .map(|c| c.to_string().parse::<usize>().unwrap())
                .collect();
            let (i, first) = chars
                .iter()
                .take(chars.len() - 1)
                .enumerate()
                .max_by(|(_, a), (_, b)| {
                    if a == b {
                        return Ordering::Greater;
                    }
                    a.cmp(b)
                })
                .unwrap();
            let second = chars.iter().skip(i + 1).max().unwrap();

            first * 10 + second
        })
        .sum()
}

fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|l| {
            let chars: Vec<usize> = l
                .trim()
                .chars()
                .map(|c| c.to_string().parse::<usize>().unwrap())
                .collect();

            let mut current = 0;
            let mut result = [0; 12];
            for i in (0..12).rev() {
                (current, result[11 - i]) = chars
                    .iter()
                    .enumerate()
                    .skip(current)
                    .take(chars.len() - i - current)
                    .map(|(j, a)| (j, *a))
                    .max_by(|(_, a), (_, b)| {
                        if a == b {
                            return Ordering::Greater;
                        }
                        a.cmp(b)
                    })
                    .unwrap();
                current += 1;
            }

            let mut r = 0;
            for (i, n) in result.iter().enumerate() {
                r += n * 10usize.pow(11 - i as u32);
            }
            r
        })
        .sum()
}
