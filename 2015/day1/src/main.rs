fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

fn part1() -> i32 {
    std::fs::read_to_string("input")
        .unwrap()
        .trim()
        .chars()
        .map(|c| if c == '(' { 1 } else { -1 })
        .sum::<i32>()
}

fn part2() -> usize {
    let mut count = 0;
    for (idx, x) in std::fs::read_to_string("input")
        .unwrap()
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
