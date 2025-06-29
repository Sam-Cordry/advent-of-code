use std::collections::VecDeque;

fn main() {
    let input = include_str!("../input");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn knot_hash_of(s: &str) -> String {
    let mut len = s
        .trim()
        .as_bytes()
        .to_vec()
        .iter()
        .map(|&n| n as usize)
        .collect::<Vec<usize>>();
    len.extend([17, 31, 73, 47, 23]);

    let mut list = (0..256).collect::<Vec<usize>>();
    let mut idx = 0;
    let mut skip = 0;

    for _ in 0..64 {
        for l in len.iter() {
            let mut new = list.split_off(idx);
            new.append(&mut list);

            let mut temp = new.split_off(*l);
            new.reverse();
            new.append(&mut temp);

            list = new.split_off(new.len() - idx);
            list.append(&mut new);

            idx += l + skip;
            idx %= list.len();
            skip += 1;
        }
    }

    let hex = list
        .chunks(16)
        .map(|c| c.iter().fold(0, |agg, n| agg ^ n) as u8)
        .collect::<Vec<u8>>();

    let mut result = String::new();
    for h in hex {
        result.push_str(&format!("{h:08b}"));
    }
    result
}

fn part1(input: &str) -> usize {
    (0..128)
        .map(|n| {
            knot_hash_of(&(input.trim().to_string() + "-" + &n.to_string()))
                .chars()
                .map(|c| c.to_string().parse::<usize>().unwrap())
                .sum::<usize>()
        })
        .sum()
}

fn get_next(current: (usize, usize), grid: &[Vec<bool>]) -> Vec<(usize, usize)> {
    let mut result = vec![];

    if current.0 != 0 && grid[current.0 - 1][current.1] {
        result.push((current.0 - 1, current.1));
    }
    if current.0 != grid.len() - 1 && grid[current.0 + 1][current.1] {
        result.push((current.0 + 1, current.1));
    }
    if current.1 != 0 && grid[current.0][current.1 - 1] {
        result.push((current.0, current.1 - 1));
    }
    if current.1 != grid.len() - 1 && grid[current.0][current.1 + 1] {
        result.push((current.0, current.1 + 1));
    }

    result
}

fn part2(input: &str) -> usize {
    let grid: Vec<Vec<bool>> = (0..128)
        .map(|n| {
            knot_hash_of(&(input.trim().to_string() + "-" + &n.to_string()))
                .chars()
                .map(|c| c == '1')
                .collect::<Vec<bool>>()
        })
        .collect();

    let mut visited = vec![vec![false; 128]; 128];
    let mut count = 0;

    for i in 0..128 {
        for j in 0..128 {
            if grid[i][j] && !visited[i][j] {
                visited[i][j] = true;
                count += 1;

                let mut queue = VecDeque::from_iter([(i, j)]);

                while let Some(current) = queue.pop_front() {
                    for n in get_next(current, &grid) {
                        if !visited[n.0][n.1] {
                            queue.push_back(n);
                            visited[n.0][n.1] = true;
                        }
                    }
                }
            }
        }
    }

    count
}
