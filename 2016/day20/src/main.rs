fn main() {
    let input = include_str!("../input");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let mut blocked = vec![false; 4294967296];

    for (start, end) in input.lines().map(|l| {
        let temp = l.trim().split('-').collect::<Vec<&str>>();
        (
            temp[0].parse::<usize>().unwrap(),
            temp[1].parse::<usize>().unwrap(),
        )
    }) {
        for ip in blocked.iter_mut().take(end + 1).skip(start) {
            *ip = true;
        }
    }

    blocked.iter().position(|ip| !ip).unwrap()
}

fn part2(input: &str) -> usize {
    let mut blocked = vec![false; 4294967296];

    for (start, end) in input.lines().map(|l| {
        let temp = l.trim().split('-').collect::<Vec<&str>>();
        (
            temp[0].parse::<usize>().unwrap(),
            temp[1].parse::<usize>().unwrap(),
        )
    }) {
        for ip in blocked.iter_mut().take(end + 1).skip(start) {
            *ip = true;
        }
    }

    blocked.iter().filter(|&ip| !ip).count()
}
