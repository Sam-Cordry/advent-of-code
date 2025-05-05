fn main() {
    let input = include_str!("../input");

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let mut lights: Vec<Vec<bool>> = vec![vec![false; 1000]; 1000];

    for l in input.lines().map(|l| l.trim()) {
        let instr = l.split_whitespace().collect::<Vec<_>>();

        if instr.len() == 4 {
            let start: Vec<usize> = instr[1]
                .split(",")
                .map(|n| n.parse::<usize>().unwrap())
                .collect();
            let end: Vec<usize> = instr[3]
                .split(",")
                .map(|n| n.parse::<usize>().unwrap())
                .collect();

            for row in lights.iter_mut().take(end[1] + 1).skip(start[1]) {
                for light in row.iter_mut().take(end[0] + 1).skip(start[0]) {
                    *light = !*light;
                }
            }
        } else {
            let start: Vec<usize> = instr[2]
                .split(",")
                .map(|n| n.parse::<usize>().unwrap())
                .collect();
            let end: Vec<usize> = instr[4]
                .split(",")
                .map(|n| n.parse::<usize>().unwrap())
                .collect();

            for row in lights.iter_mut().take(end[1] + 1).skip(start[1]) {
                for light in row.iter_mut().take(end[0] + 1).skip(start[0]) {
                    *light = instr[1] == "on";
                }
            }
        }
    }

    lights
        .into_iter()
        .map(|r| r.into_iter().filter(|v| *v).count())
        .sum()
}

fn part2(input: &str) -> usize {
    let mut lights: Vec<Vec<usize>> = vec![vec![0; 1000]; 1000];

    for l in input.lines().map(|l| l.trim()) {
        let instr = l.split_whitespace().collect::<Vec<_>>();

        if instr.len() == 4 {
            let start: Vec<usize> = instr[1]
                .split(",")
                .map(|n| n.parse::<usize>().unwrap())
                .collect();
            let end: Vec<usize> = instr[3]
                .split(",")
                .map(|n| n.parse::<usize>().unwrap())
                .collect();

            for row in lights.iter_mut().take(end[1] + 1).skip(start[1]) {
                for light in row.iter_mut().take(end[0] + 1).skip(start[0]) {
                    *light += 2;
                }
            }
        } else {
            let start: Vec<usize> = instr[2]
                .split(",")
                .map(|n| n.parse::<usize>().unwrap())
                .collect();
            let end: Vec<usize> = instr[4]
                .split(",")
                .map(|n| n.parse::<usize>().unwrap())
                .collect();

            for row in lights.iter_mut().take(end[1] + 1).skip(start[1]) {
                for light in row.iter_mut().take(end[0] + 1).skip(start[0]) {
                    if instr[1] == "on" {
                        *light += 1;
                    } else if *light > 0 {
                        *light -= 1;
                    }
                }
            }
        }
    }

    lights
        .into_iter()
        .map(|r| r.into_iter().sum::<usize>())
        .sum()
}
