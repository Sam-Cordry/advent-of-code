fn main() {
    let input = include_str!("../input");

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|l| {
            let dims: Vec<usize> = l.split("x").map(|n| n.parse().unwrap()).collect();
            let mut areas: Vec<usize> =
                vec![dims[0] * dims[1], dims[1] * dims[2], dims[0] * dims[2]];
            areas.sort();
            2 * (areas[0] + areas[1] + areas[2]) + areas[0]
        })
        .sum()
}

fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|l| {
            let mut dims: Vec<usize> = l.split("x").map(|n| n.parse().unwrap()).collect();
            dims.sort();
            2 * (dims[0] + dims[1]) + dims[0] * dims[1] * dims[2]
        })
        .sum()
}
