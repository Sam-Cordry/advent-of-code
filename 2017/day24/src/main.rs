fn main() {
    let input = include_str!("../input");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn dfs_max_str(current: usize, remaining: Vec<(usize, usize)>) -> usize {
    let mut max = 0;

    for next in remaining.iter() {
        if next.0 == current || next.1 == current {
            let temp = dfs_max_str(
                if next.0 == current { next.1 } else { next.0 },
                remaining.iter().copied().filter(|c| c != next).collect(),
            ) + next.0
                + next.1;
            if temp > max {
                max = temp;
            }
        }
    }

    max
}

fn part1(input: &str) -> usize {
    dfs_max_str(
        0,
        input
            .lines()
            .map(|l| {
                let mut temp = l.trim().split('/');
                (
                    temp.next().unwrap().parse().unwrap(),
                    temp.next().unwrap().parse().unwrap(),
                )
            })
            .collect(),
    )
}

fn dfs_max_len(current: usize, remaining: Vec<(usize, usize)>, depth: usize) -> (usize, usize) {
    let mut max = (depth, 0);

    for next in remaining.iter() {
        if next.0 == current || next.1 == current {
            let mut temp = dfs_max_len(
                if next.0 == current { next.1 } else { next.0 },
                remaining.iter().copied().filter(|c| c != next).collect(),
                depth + 1,
            );
            temp.1 += next.0 + next.1;
            if temp.0 > max.0 || (temp.0 == max.0 && temp.1 > max.1) {
                max = temp;
            }
        }
    }

    max
}

fn part2(input: &str) -> usize {
    dfs_max_len(
        0,
        input
            .lines()
            .map(|l| {
                let mut temp = l.trim().split('/');
                (
                    temp.next().unwrap().parse().unwrap(),
                    temp.next().unwrap().parse().unwrap(),
                )
            })
            .collect(),
        0,
    )
    .1
}
