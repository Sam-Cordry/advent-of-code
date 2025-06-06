fn main() {
    let input = include_str!("../input");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let (mut a, mut b) = input
        .lines()
        .map(|l| {
            l.chars()
                .rev()
                .take_while(|&c| c != ' ')
                .collect::<Vec<char>>()
                .iter()
                .rev()
                .collect::<String>()
                .parse::<usize>()
                .unwrap()
        })
        .collect::<Vec<usize>>()
        .windows(2)
        .map(|w| (w[0], w[1]))
        .next()
        .unwrap();

    let mut result = 0;

    for _ in 0..40000000 {
        a *= 16807;
        a %= 2147483647;
        b *= 48271;
        b %= 2147483647;

        if format!("{a:32b}")[16..] == format!("{b:32b}")[16..] {
            result += 1;
        }
    }

    result
}

fn part2(input: &str) -> usize {
    let (mut a, mut b) = input
        .lines()
        .map(|l| {
            l.chars()
                .rev()
                .take_while(|&c| c != ' ')
                .collect::<Vec<char>>()
                .iter()
                .rev()
                .collect::<String>()
                .parse::<usize>()
                .unwrap()
        })
        .collect::<Vec<usize>>()
        .windows(2)
        .map(|w| (w[0], w[1]))
        .next()
        .unwrap();

    let mut result = 0;
    let mut i = 0;

    while i < 5000000 {
        a *= 16807;
        a %= 2147483647;
        b *= 48271;
        b %= 2147483647;

        while a % 4 != 0 || b % 8 != 0 {
            if a % 4 != 0 {
                a *= 16807;
                a %= 2147483647;
            } else {
                b *= 48271;
                b %= 2147483647;
            }
        }

        if format!("{a:32b}")[16..] == format!("{b:32b}")[16..] {
            result += 1;
        }

        i += 1;
    }

    result
}
