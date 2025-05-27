fn main() {
    let input = include_str!("../input");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let state = Vec::from_iter(input.lines().map(|l| {
        let split = l.split_whitespace().collect::<Vec<&str>>();
        (
            split[3].parse::<usize>().unwrap(),
            split[11][..split[11].len() - 1].parse::<usize>().unwrap(),
        )
    }));

    let mut time = 0;
    loop {
        let mut valid = true;
        for (idx, (max, start)) in state.iter().enumerate() {
            if (start + time + idx + 1) % max != 0 {
                valid = false;
                break;
            }
        }

        if valid {
            return time;
        }

        time += 1;
    }
}

fn part2(input: &str) -> usize {
    let mut state = Vec::from_iter(input.lines().map(|l| {
        let split = l.split_whitespace().collect::<Vec<&str>>();
        (
            split[3].parse::<usize>().unwrap(),
            split[11][..split[11].len() - 1].parse::<usize>().unwrap(),
        )
    }));
    state.push((11, 0));

    let mut time = 0;
    loop {
        let mut valid = true;
        for (idx, (max, start)) in state.iter().enumerate() {
            if (start + time + idx + 1) % max != 0 {
                valid = false;
                break;
            }
        }

        if valid {
            return time;
        }

        time += 1;
    }
}
