fn main() {
    let input = include_str!("../input");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let mut fabric: Vec<Vec<usize>> = vec![vec![0; 1000]; 1000];

    for l in input.lines().map(|l| {
        l.trim()
            .chars()
            .skip_while(|c| *c != '@')
            .skip(2)
            .collect::<String>()
    }) {
        for i in (l
            .chars()
            .take_while(|c| *c != ',')
            .collect::<String>()
            .parse::<usize>()
            .unwrap()..)
            .take(
                l.chars()
                    .skip_while(|c| *c != ':')
                    .skip(2)
                    .take_while(|c| *c != 'x')
                    .collect::<String>()
                    .parse()
                    .unwrap(),
            )
        {
            for j in (l
                .chars()
                .skip_while(|c| *c != ',')
                .skip(1)
                .take_while(|c| *c != ':')
                .collect::<String>()
                .parse::<usize>()
                .unwrap()..)
                .take(
                    l.chars()
                        .skip_while(|c| *c != 'x')
                        .skip(1)
                        .collect::<String>()
                        .parse()
                        .unwrap(),
                )
            {
                fabric[i][j] += 1;
            }
        }
    }

    fabric
        .iter()
        .map(|r| r.iter().filter(|x| **x > 1).count())
        .sum()
}

fn part2(input: &str) -> usize {
    let mut fabric: Vec<Vec<Vec<usize>>> = vec![vec![vec![]; 1000]; 1000];

    for (idx, l) in input
        .lines()
        .map(|l| {
            l.trim()
                .chars()
                .skip_while(|c| *c != '@')
                .skip(2)
                .collect::<String>()
        })
        .enumerate()
    {
        for i in (l
            .chars()
            .take_while(|c| *c != ',')
            .collect::<String>()
            .parse::<usize>()
            .unwrap()..)
            .take(
                l.chars()
                    .skip_while(|c| *c != ':')
                    .skip(2)
                    .take_while(|c| *c != 'x')
                    .collect::<String>()
                    .parse()
                    .unwrap(),
            )
        {
            for j in (l
                .chars()
                .skip_while(|c| *c != ',')
                .skip(1)
                .take_while(|c| *c != ':')
                .collect::<String>()
                .parse::<usize>()
                .unwrap()..)
                .take(
                    l.chars()
                        .skip_while(|c| *c != 'x')
                        .skip(1)
                        .collect::<String>()
                        .parse()
                        .unwrap(),
                )
            {
                fabric[i][j].push(idx + 1);
            }
        }
    }

    for i in 1..input.lines().count() + 1 {
        if fabric.iter().all(|r| {
            r.iter()
                .all(|v| (v.contains(&i) && v.len() == 1) || !v.contains(&i))
        }) {
            return i;
        }
    }

    0
}
