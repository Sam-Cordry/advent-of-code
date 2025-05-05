fn main() {
    let input = include_str!("../input");

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn step(current: &[Vec<char>]) -> Vec<Vec<char>> {
    let mut next: Vec<Vec<char>> = vec![vec!['.'; 100]; 100];

    for (r_idx, row) in current.iter().enumerate() {
        for (c_idx, c) in row.iter().enumerate() {
            let mut count = 0;
            for i in 0..=2 {
                for j in 0..=2 {
                    if (i != 1 || j != 1)
                        && (r_idx != 0 || i != 0)
                        && (r_idx != 99 || i != 2)
                        && (c_idx != 0 || j != 0)
                        && (c_idx != 99 || j != 2)
                        && current[r_idx + i - 1][c_idx + j - 1] == '#'
                    {
                        count += 1;
                    }
                }
            }

            if *c == '#' && count != 2 && count != 3 {
                next[r_idx][c_idx] = '.';
            } else if *c == '.' && count == 3 {
                next[r_idx][c_idx] = '#';
            } else {
                next[r_idx][c_idx] = *c;
            }
        }
    }

    next
}

fn part1(input: &str) -> usize {
    let mut current: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    for _ in 0..100 {
        current = step(&current);
    }

    current
        .iter()
        .map(|l| l.iter().filter(|c| *c == &'#').count())
        .sum()
}

fn part2(input: &str) -> usize {
    let mut current: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    for _ in 0..100 {
        current[0][0] = '#';
        current[99][0] = '#';
        current[0][99] = '#';
        current[99][99] = '#';

        current = step(&current);
    }
    current[0][0] = '#';
    current[99][0] = '#';
    current[0][99] = '#';
    current[99][99] = '#';

    current
        .iter()
        .map(|l| l.iter().filter(|c| *c == &'#').count())
        .sum()
}
