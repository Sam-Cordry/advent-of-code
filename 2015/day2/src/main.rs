fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

fn part1() -> usize {
    std::fs::read_to_string("input")
        .unwrap()
        .trim()
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

fn part2() -> usize {
    std::fs::read_to_string("input")
        .unwrap()
        .trim()
        .lines()
        .map(|l| {
            let mut dims: Vec<usize> = l.split("x").map(|n| n.parse().unwrap()).collect();
            dims.sort();
            2 * (dims[0] + dims[1]) + dims[0] * dims[1] * dims[2]
        })
        .sum()
}
