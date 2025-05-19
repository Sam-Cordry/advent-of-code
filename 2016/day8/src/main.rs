use std::collections::VecDeque;

fn main() {
    let input = include_str!("../input");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let mut lights: Vec<VecDeque<bool>> = vec![VecDeque::new(); 6];

    for row in lights.iter_mut() {
        for _ in 0..50 {
            row.push_back(false);
        }
    }

    for line in input
        .lines()
        .map(|l| l.split_whitespace().collect::<Vec<&str>>())
    {
        if line[0] == "rect" {
            for row in lights.iter_mut().take(
                line[1]
                    .split('x')
                    .next_back()
                    .unwrap()
                    .parse::<usize>()
                    .unwrap(),
            ) {
                for light in row
                    .iter_mut()
                    .take(line[1].split('x').next().unwrap().parse::<usize>().unwrap())
                {
                    *light = true;
                }
            }
        } else if line[1] == "row" {
            let row = line[2]
                .split('=')
                .next_back()
                .unwrap()
                .parse::<usize>()
                .unwrap();
            let offset = line[4].parse::<usize>().unwrap();

            let mut runoff = lights.get_mut(row).unwrap().split_off(50 - offset);
            runoff.extend(lights.get(row).unwrap());
            lights[row] = runoff;
        } else if line[1] == "column" {
            let col = line[2]
                .split('=')
                .next_back()
                .unwrap()
                .parse::<usize>()
                .unwrap();
            let offset = line[4].parse::<usize>().unwrap();

            let mut column = lights.iter().map(|r| r[col]).collect::<Vec<bool>>();
            let mut runoff = column.split_off(6 - offset);
            runoff.extend(column);

            for (idx, row) in lights.iter_mut().enumerate() {
                row[col] = runoff[idx];
            }
        }
    }

    lights
        .iter()
        .map(|r| r.iter().filter(|l| **l).count())
        .sum()
}

fn part2(input: &str) -> String {
    let mut lights: Vec<VecDeque<bool>> = vec![VecDeque::new(); 6];

    for row in lights.iter_mut() {
        for _ in 0..50 {
            row.push_back(false);
        }
    }

    for line in input
        .lines()
        .map(|l| l.split_whitespace().collect::<Vec<&str>>())
    {
        if line[0] == "rect" {
            for row in lights.iter_mut().take(
                line[1]
                    .split('x')
                    .next_back()
                    .unwrap()
                    .parse::<usize>()
                    .unwrap(),
            ) {
                for light in row
                    .iter_mut()
                    .take(line[1].split('x').next().unwrap().parse::<usize>().unwrap())
                {
                    *light = true;
                }
            }
        } else if line[1] == "row" {
            let row = line[2]
                .split('=')
                .next_back()
                .unwrap()
                .parse::<usize>()
                .unwrap();
            let offset = line[4].parse::<usize>().unwrap();

            let mut runoff = lights.get_mut(row).unwrap().split_off(50 - offset);
            runoff.extend(lights.get(row).unwrap());
            lights[row] = runoff;
        } else if line[1] == "column" {
            let col = line[2]
                .split('=')
                .next_back()
                .unwrap()
                .parse::<usize>()
                .unwrap();
            let offset = line[4].parse::<usize>().unwrap();

            let mut column = lights.iter().map(|r| r[col]).collect::<Vec<bool>>();
            let mut runoff = column.split_off(6 - offset);
            runoff.extend(column);

            for (idx, row) in lights.iter_mut().enumerate() {
                row[col] = runoff[idx];
            }
        }
    }

    let mut result = String::from("\n");

    for line in lights.iter().map(|r| {
        r.iter()
            .map(|b| if *b { '#' } else { '.' })
            .collect::<String>()
    }) {
        result.push_str(&line);
        result.push('\n');
    }

    result
}
