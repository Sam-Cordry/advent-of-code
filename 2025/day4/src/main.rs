fn main() {
    let input = include_str!("../input");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let rolls: Vec<Vec<bool>> = input
        .lines()
        .map(|l| l.chars().map(|c| c == '@').collect())
        .collect();

    let mut result = 0;
    for i in 0..rolls.len() {
        for j in 0..rolls[0].len() {
            if rolls[i][j]
                && [
                    *rolls
                        .get(i.overflowing_sub(1).0)
                        .unwrap_or(&vec![])
                        .get(j.overflowing_sub(1).0)
                        .unwrap_or(&false),
                    *rolls
                        .get(i.overflowing_sub(1).0)
                        .unwrap_or(&vec![])
                        .get(j)
                        .unwrap_or(&false),
                    *rolls
                        .get(i.overflowing_sub(1).0)
                        .unwrap_or(&vec![])
                        .get(j + 1)
                        .unwrap_or(&false),
                    *rolls[i].get(j.overflowing_sub(1).0).unwrap_or(&false),
                    *rolls[i].get(j + 1).unwrap_or(&false),
                    *rolls
                        .get(i + 1)
                        .unwrap_or(&vec![])
                        .get(j.overflowing_sub(1).0)
                        .unwrap_or(&false),
                    *rolls.get(i + 1).unwrap_or(&vec![]).get(j).unwrap_or(&false),
                    *rolls
                        .get(i + 1)
                        .unwrap_or(&vec![])
                        .get(j + 1)
                        .unwrap_or(&false),
                ]
                .iter()
                .filter(|c| **c)
                .count()
                    < 4
            {
                result += 1;
            }
        }
    }
    result
}

fn part2(input: &str) -> usize {
    let mut rolls: Vec<Vec<bool>> = input
        .lines()
        .map(|l| l.chars().map(|c| c == '@').collect())
        .collect();

    let mut result = 0;
    loop {
        let mut new: Vec<Vec<bool>> = rolls.iter().map(|l| l.to_vec()).collect();

        let mut sum = 0;
        for i in 0..rolls.len() {
            for j in 0..rolls[0].len() {
                if rolls[i][j]
                    && [
                        *rolls
                            .get(i.overflowing_sub(1).0)
                            .unwrap_or(&vec![])
                            .get(j.overflowing_sub(1).0)
                            .unwrap_or(&false),
                        *rolls
                            .get(i.overflowing_sub(1).0)
                            .unwrap_or(&vec![])
                            .get(j)
                            .unwrap_or(&false),
                        *rolls
                            .get(i.overflowing_sub(1).0)
                            .unwrap_or(&vec![])
                            .get(j + 1)
                            .unwrap_or(&false),
                        *rolls[i].get(j.overflowing_sub(1).0).unwrap_or(&false),
                        *rolls[i].get(j + 1).unwrap_or(&false),
                        *rolls
                            .get(i + 1)
                            .unwrap_or(&vec![])
                            .get(j.overflowing_sub(1).0)
                            .unwrap_or(&false),
                        *rolls.get(i + 1).unwrap_or(&vec![]).get(j).unwrap_or(&false),
                        *rolls
                            .get(i + 1)
                            .unwrap_or(&vec![])
                            .get(j + 1)
                            .unwrap_or(&false),
                    ]
                    .iter()
                    .filter(|c| **c)
                    .count()
                        < 4
                {
                    sum += 1;
                    new[i][j] = false;
                }
            }
        }
        if sum == 0 {
            break;
        }

        result += sum;
        rolls = new;
    }
    result
}
