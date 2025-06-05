fn main() {
    let input = include_str!("../input");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let end = input
        .split(',')
        .map(|d| match d.trim() {
            "n" => (1_i32, -1_i32, 0_i32),
            "ne" => (1_i32, 0_i32, -1_i32),
            "se" => (0_i32, 1_i32, -1_i32),
            "s" => (-1_i32, 1_i32, 0_i32),
            "sw" => (-1_i32, 0_i32, 1_i32),
            "nw" => (0_i32, -1_i32, 1_i32),
            _ => unreachable!("invalid direction"),
        })
        .fold((0, 0, 0), |mut acc, x| {
            acc.0 += x.0;
            acc.1 += x.1;
            acc.2 += x.2;
            acc
        });

    (end.0.abs() + end.1.abs() + end.2.abs()) as usize / 2
}

fn part2(input: &str) -> usize {
    let mut max = 0;

    input
        .split(',')
        .map(|d| match d.trim() {
            "n" => (1_i32, -1_i32, 0_i32),
            "ne" => (1_i32, 0_i32, -1_i32),
            "se" => (0_i32, 1_i32, -1_i32),
            "s" => (-1_i32, 1_i32, 0_i32),
            "sw" => (-1_i32, 0_i32, 1_i32),
            "nw" => (0_i32, -1_i32, 1_i32),
            _ => unreachable!("invalid direction"),
        })
        .fold((0, 0, 0), |mut acc, x| {
            acc.0 += x.0;
            acc.1 += x.1;
            acc.2 += x.2;

            if (acc.0.abs() + acc.1.abs() + acc.2.abs()) as usize / 2 > max {
                max = (acc.0.abs() + acc.1.abs() + acc.2.abs()) as usize / 2;
            }

            acc
        });

    max
}
