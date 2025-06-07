fn main() {
    let input = include_str!("../input");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let offset = input.trim().parse::<usize>().unwrap();

    let mut buffer = vec![0];
    let mut i = 0;

    for v in 1..=2017 {
        i += offset;
        i %= buffer.len();
        i += 1;
        buffer.insert(i, v);
    }

    buffer[i + 1]
}

fn part2(input: &str) -> usize {
    let offset = input.trim().parse::<usize>().unwrap();

    let mut i = 0;
    let mut result = 0;

    for v in 1..=50000000 {
        i += offset;
        i %= v;
        i += 1;

        if i == 1 {
            result = v;
        }
    }

    result
}
