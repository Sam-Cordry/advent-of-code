fn main() {
    let input = include_str!("../input");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    usize::from_str_radix(
        &(format!("{:b}", input.trim().parse::<usize>().unwrap())
            .chars()
            .skip(1)
            .collect::<String>()
            + "1"),
        2,
    )
    .unwrap()
}

fn part2(input: &str) -> usize {
    let num = input.trim().parse::<usize>().unwrap();

    let mut chunk = 1;
    while chunk < num {
        chunk *= 3;
    }

    if chunk >= num {
        chunk /= 3;
    }

    num - chunk + num.saturating_sub(2 * chunk)
}
