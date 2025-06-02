fn main() {
    let input = include_str!("../input");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    (input.trim().to_string() + &input[0..1])
        .chars()
        .collect::<Vec<char>>()
        .windows(2)
        .filter_map(|w| {
            if w[0] == w[1] {
                Some(w[0].to_string().parse::<usize>().unwrap())
            } else {
                None
            }
        })
        .sum()
}

fn part2(input: &str) -> usize {
    input
        .trim()
        .to_string()
        .chars()
        .collect::<Vec<char>>()
        .iter()
        .enumerate()
        .map(|(idx, &c)| {
            (
                c,
                input
                    .trim()
                    .chars()
                    .nth((idx + input.len() / 2) % input.trim().len())
                    .unwrap(),
            )
        })
        .filter_map(|(a, b)| {
            if a == b {
                Some(a.to_string().parse::<usize>().unwrap())
            } else {
                None
            }
        })
        .sum()

    // 0
}
