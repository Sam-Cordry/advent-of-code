use std::collections::HashMap;

struct Disc {
    weight: usize,
    holds: Vec<String>,
}

impl Disc {
    fn get_weight(&self, discs: &HashMap<&str, Disc>) -> usize {
        if self.holds.is_empty() {
            self.weight
        } else {
            self.weight
                + self
                    .holds
                    .iter()
                    .map(|d| discs.get(d.as_str()).unwrap().get_weight(discs))
                    .sum::<usize>()
        }
    }

    fn is_balanced(&self, discs: &HashMap<&str, Disc>) -> bool {
        self.holds
            .iter()
            .map(|d| discs.get(d.as_str()).unwrap().get_weight(discs))
            .collect::<Vec<usize>>()
            .windows(2)
            .all(|w| w[0] == w[1])
    }
}

fn main() {
    let input = include_str!("../input");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> String {
    for name in input.lines().map(|l| l.split_whitespace().next().unwrap()) {
        if input.lines().filter(|l| l.contains(name)).count() == 1 {
            return name.to_string();
        }
    }

    String::new()
}

fn part2(input: &str) -> usize {
    let discs: HashMap<&str, Disc> = HashMap::from_iter(input.lines().map(|l| {
        let split = l.split_whitespace().collect::<Vec<&str>>();

        (
            split[0],
            if split.len() > 2 {
                Disc {
                    weight: split[1][1..split[1].len() - 1].parse::<usize>().unwrap(),
                    holds: split
                        .iter()
                        .skip(3)
                        .map(|s| {
                            if let Some(d) = s.strip_suffix(',') {
                                d.to_string()
                            } else {
                                s.to_string()
                            }
                        })
                        .collect::<Vec<String>>(),
                }
            } else {
                Disc {
                    weight: split[1][1..split[1].len() - 1].parse::<usize>().unwrap(),
                    holds: vec![],
                }
            },
        )
    }));

    let mut current = discs.get(part1(input).as_str()).unwrap();
    loop {
        if !current.is_balanced(&discs)
            && current
                .holds
                .iter()
                .map(|h| discs.get(h.as_str()).unwrap().is_balanced(&discs))
                .any(|b| !b)
        {
            current = discs
                .get(
                    current
                        .holds
                        .iter()
                        .find(|h| !discs.get(h.as_str()).unwrap().is_balanced(&discs))
                        .unwrap()
                        .as_str(),
                )
                .unwrap();
            continue;
        }

        let weights = current
            .holds
            .iter()
            .map(|h| discs.get(h.as_str()).unwrap().get_weight(&discs) as i32)
            .collect::<Vec<i32>>();

        let (idx, offset) = if weights[0] != weights[1] && weights[0] != weights[2] {
            weights
                .iter()
                .map(|w| w - weights[1])
                .enumerate()
                .find(|(_, w)| *w != 0)
                .unwrap()
        } else {
            weights
                .iter()
                .map(|w| w - weights[0])
                .enumerate()
                .find(|(_, w)| *w != 0)
                .unwrap()
        };

        return (discs.get(current.holds[idx].as_str()).unwrap().weight as i32 - offset) as usize;
    }
}
