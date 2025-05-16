fn main() {
    let input = include_str!("../input");
    println!("Part 1: {}", part1(input));
}

fn part1(input: &str) -> usize {
    let (row, col) = input
        .split_whitespace()
        .collect::<Vec<&str>>()
        .windows(3)
        .filter(|w| {
            w[0][..w[0].len() - 1].parse::<usize>().is_ok()
                && w[2][..w[2].len() - 1].parse::<usize>().is_ok()
        })
        .map(|w| {
            (
                w[0][..w[0].len() - 1].parse::<usize>().unwrap() - 1,
                w[2][..w[2].len() - 1].parse::<usize>().unwrap() - 1,
            )
        })
        .next()
        .unwrap();

    let mut count = 0;
    let mut t_row = 0;
    let mut t_col = 0;
    while t_row != row || t_col != col {
        count += 1;

        if t_row == 0 {
            t_row = t_col + 1;
            t_col = 0;
        } else {
            t_row -= 1;
            t_col += 1;
        }
    }

    let mut result = 20151125;
    for _ in 0..count {
        result = (result * 252533) % 33554393;
    }

    result
}
