fn main() {
    let input = include_str!("../input");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let mut jumps = input
        .lines()
        .map(|l| l.trim().parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    let mut idx: i32 = 0;
    let mut steps = 0;

    while idx < jumps.len() as i32 && idx >= 0 {
        jumps[idx as usize] += 1;
        idx += jumps[idx as usize] - 1;
        steps += 1;
    }

    steps
}

fn part2(input: &str) -> usize {
    let mut jumps = input
        .lines()
        .map(|l| l.trim().parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    let mut idx: i32 = 0;
    let mut steps = 0;

    while idx < jumps.len() as i32 && idx >= 0 {
        if jumps[idx as usize] >= 3 {
            jumps[idx as usize] -= 1;
            idx += jumps[idx as usize] + 1;
        } else {
            jumps[idx as usize] += 1;
            idx += jumps[idx as usize] - 1;
        }
        steps += 1;
    }

    steps
}
