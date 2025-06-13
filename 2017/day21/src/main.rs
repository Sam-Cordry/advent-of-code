use std::collections::HashMap;

fn main() {
    let input = include_str!("../input");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn permutations_of(grid: &[Vec<char>]) -> Vec<Vec<Vec<char>>> {
    let left = (0..grid[0].len())
        .map(|c| {
            (0..grid.len())
                .map(|r| grid[grid.len() - 1 - r][c])
                .collect::<Vec<char>>()
        })
        .collect::<Vec<Vec<char>>>();
    let up = (0..left[0].len())
        .map(|c| {
            (0..left.len())
                .map(|r| left[left.len() - 1 - r][c])
                .collect::<Vec<char>>()
        })
        .collect::<Vec<Vec<char>>>();
    let right = (0..up[0].len())
        .map(|c| {
            (0..up.len())
                .map(|r| up[up.len() - 1 - r][c])
                .collect::<Vec<char>>()
        })
        .collect::<Vec<Vec<char>>>();

    vec![
        grid.iter()
            .map(|r| r.iter().rev().copied().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>(),
        left.clone(),
        left.iter()
            .map(|r| r.iter().rev().copied().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>(),
        up.clone(),
        up.iter()
            .map(|r| r.iter().rev().copied().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>(),
        right.clone(),
        right
            .iter()
            .map(|r| r.iter().rev().copied().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>(),
    ]
}

fn next(current: &mut Vec<Vec<char>>, rules: &HashMap<Vec<Vec<char>>, Vec<Vec<char>>>) {
    let size = current.len();
    if size % 2 == 0 {
        let mut replace: Vec<Vec<Vec<char>>> = vec![vec![]; (size / 2).pow(2)];
        for i in (0..size).collect::<Vec<usize>>().chunks(2) {
            for j in (0..size).collect::<Vec<usize>>().chunks(2) {
                replace[(i[0] / 2) * (size / 2) + (j[0] / 2)] = rules
                    .get(&vec![
                        vec![current[i[0]][j[0]], current[i[0]][j[1]]],
                        vec![current[i[1]][j[0]], current[i[1]][j[1]]],
                    ])
                    .unwrap()
                    .to_vec();
            }
        }

        let new_size = (size / 2) * 3;
        *current = vec![vec![' '; new_size]; new_size];
        for i in 0..new_size {
            for j in 0..new_size {
                current[i][j] = replace[(i / 3) * (size / 2) + (j / 3)][i % 3][j % 3];
            }
        }
    } else {
        let mut replace: Vec<Vec<Vec<char>>> = vec![vec![]; (size / 3).pow(2)];
        for i in (0..size).collect::<Vec<usize>>().chunks(3) {
            for j in (0..size).collect::<Vec<usize>>().chunks(3) {
                replace[(i[0] / 3) * (size / 3) + (j[0] / 3)] = rules
                    .get(&vec![
                        vec![
                            current[i[0]][j[0]],
                            current[i[0]][j[1]],
                            current[i[0]][j[2]],
                        ],
                        vec![
                            current[i[1]][j[0]],
                            current[i[1]][j[1]],
                            current[i[1]][j[2]],
                        ],
                        vec![
                            current[i[2]][j[0]],
                            current[i[2]][j[1]],
                            current[i[2]][j[2]],
                        ],
                    ])
                    .unwrap()
                    .to_vec();
            }
        }

        let new_size = (size / 3) * 4;
        *current = vec![vec![' '; new_size]; new_size];
        for i in 0..new_size {
            for j in 0..new_size {
                current[i][j] = replace[(i / 4) * (size / 3) + (j / 4)][i % 4][j % 4];
            }
        }
    }
}

fn part1(input: &str) -> usize {
    let mut current = vec![
        vec!['.', '#', '.'],
        vec!['.', '.', '#'],
        vec!['#', '#', '#'],
    ];

    let mut rules: HashMap<Vec<Vec<char>>, Vec<Vec<char>>> = input
        .lines()
        .flat_map(|l| l.split("=>").collect::<Vec<&str>>())
        .map(|g| {
            g.trim()
                .split('/')
                .map(|r| r.chars().collect::<Vec<char>>())
                .collect::<Vec<Vec<char>>>()
        })
        .collect::<Vec<Vec<Vec<char>>>>()
        .chunks(2)
        .map(|ch| (ch[0].clone(), ch[1].clone()))
        .collect();

    for e in rules.clone().iter() {
        for p in permutations_of(e.0) {
            rules.insert(p, e.1.clone());
        }
    }

    for _ in 0..5 {
        next(&mut current, &rules);
    }

    current
        .iter()
        .map(|r| r.iter().filter(|&&p| p == '#').count())
        .sum()
}

fn part2(input: &str) -> usize {
    let mut current = vec![
        vec!['.', '#', '.'],
        vec!['.', '.', '#'],
        vec!['#', '#', '#'],
    ];

    let mut rules: HashMap<Vec<Vec<char>>, Vec<Vec<char>>> = input
        .lines()
        .flat_map(|l| l.split("=>").collect::<Vec<&str>>())
        .map(|g| {
            g.trim()
                .split('/')
                .map(|r| r.chars().collect::<Vec<char>>())
                .collect::<Vec<Vec<char>>>()
        })
        .collect::<Vec<Vec<Vec<char>>>>()
        .chunks(2)
        .map(|ch| (ch[0].clone(), ch[1].clone()))
        .collect();

    for e in rules.clone().iter() {
        for p in permutations_of(e.0) {
            rules.insert(p, e.1.clone());
        }
    }

    for _ in 0..18 {
        next(&mut current, &rules);
    }

    current
        .iter()
        .map(|r| r.iter().filter(|&&p| p == '#').count())
        .sum()
}
