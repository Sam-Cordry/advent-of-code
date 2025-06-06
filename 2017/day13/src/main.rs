use std::collections::HashMap;

fn main() {
    let input = include_str!("../input");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let layers: HashMap<usize, usize> = HashMap::from_iter(input.lines().map(|l| {
        let split = l.split(':').collect::<Vec<&str>>();
        (split[0].parse().unwrap(), split[1].trim().parse().unwrap())
    }));

    let mut result = 0;

    for time in 0..=*layers.keys().max().unwrap() {
        if layers.contains_key(&time) && time % ((layers[&time] - 1) * 2) == 0 {
            result += time * layers[&time];
        }
    }

    result
}

fn part2(input: &str) -> usize {
    let layers: HashMap<usize, usize> = HashMap::from_iter(input.lines().map(|l| {
        let split = l.split(':').collect::<Vec<&str>>();
        (split[0].parse().unwrap(), split[1].trim().parse().unwrap())
    }));

    for delay in 0.. {
        let mut valid = true;
        for layer in layers.clone() {
            if (layer.0 + delay) % ((layer.1 - 1) * 2) == 0 {
                valid = false;
            }
        }

        if valid {
            return delay;
        }
    }

    0
}
