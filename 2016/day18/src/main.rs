fn main() {
    let input = include_str!("../input");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let mut tiles = vec![vec!['.'; input.trim().len()]; 40];
    tiles[0] = input.trim().chars().collect::<Vec<char>>();

    for i in 1..40 {
        for j in 0..tiles[0].len() {
            let reference = tiles[i - 1].clone();
            tiles[i][j] = if (*reference.get(j - 1).unwrap_or(&'.') == '^'
                && *reference.get(j).unwrap() == '^'
                && *reference.get(j + 1).unwrap_or(&'.') == '.')
                || (*reference.get(j - 1).unwrap_or(&'.') == '.'
                    && *reference.get(j).unwrap() == '^'
                    && *reference.get(j + 1).unwrap_or(&'.') == '^')
                || (*reference.get(j - 1).unwrap_or(&'.') == '^'
                    && *reference.get(j).unwrap() == '.'
                    && *reference.get(j + 1).unwrap_or(&'.') == '.')
                || (*reference.get(j - 1).unwrap_or(&'.') == '.'
                    && *reference.get(j).unwrap() == '.'
                    && *reference.get(j + 1).unwrap_or(&'.') == '^')
            {
                '^'
            } else {
                '.'
            };
        }
    }

    tiles
        .iter()
        .map(|r| {
            r.iter()
                .map(|t| if *t == '.' { 1 } else { 0 })
                .sum::<usize>()
        })
        .sum()
}

fn part2(input: &str) -> usize {
    let mut tiles = vec![vec!['.'; input.trim().len()]; 400000];
    tiles[0] = input.trim().chars().collect::<Vec<char>>();

    for i in 1..400000 {
        for j in 0..tiles[0].len() {
            let reference = tiles[i - 1].clone();
            tiles[i][j] = if (*reference.get(j - 1).unwrap_or(&'.') == '^'
                && *reference.get(j).unwrap() == '^'
                && *reference.get(j + 1).unwrap_or(&'.') == '.')
                || (*reference.get(j - 1).unwrap_or(&'.') == '.'
                    && *reference.get(j).unwrap() == '^'
                    && *reference.get(j + 1).unwrap_or(&'.') == '^')
                || (*reference.get(j - 1).unwrap_or(&'.') == '^'
                    && *reference.get(j).unwrap() == '.'
                    && *reference.get(j + 1).unwrap_or(&'.') == '.')
                || (*reference.get(j - 1).unwrap_or(&'.') == '.'
                    && *reference.get(j).unwrap() == '.'
                    && *reference.get(j + 1).unwrap_or(&'.') == '^')
            {
                '^'
            } else {
                '.'
            };
        }
    }

    tiles
        .iter()
        .map(|r| {
            r.iter()
                .map(|t| if *t == '.' { 1 } else { 0 })
                .sum::<usize>()
        })
        .sum()
}
