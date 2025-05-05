fn main() {
    let input = include_str!("../input");

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> i32 {
    input
        .trim()
        .chars()
        .map(|c| if c == '(' { 1 } else { -1 })
        .sum::<i32>()
}

fn part2(input: &str) -> usize {
    let mut count = 0;
    for (idx, x) in input
        .trim()
        .chars()
        .map(|c| if c == '(' { 1 } else { -1 })
        .enumerate()
    {
        count += x;
        if count < 0 {
            return idx + 1;
        }
    }
    0
}
