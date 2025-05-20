use std::collections::HashMap;

#[derive(Default, Clone)]
struct Container {
    is_bot: bool,
    low: Option<String>,
    high: Option<String>,
    values: Vec<usize>,
    executed: bool,
}

impl Container {
    fn give(&mut self, value: usize) {
        self.values.push(value);
    }

    fn execute(&self, containers: &mut HashMap<String, Container>) -> bool {
        if self.is_bot && self.values.len() == 2 {
            if self.values[0] < self.values[1] {
                containers
                    .get_mut(&self.low.clone().unwrap())
                    .unwrap()
                    .give(self.values[0]);
                containers
                    .get_mut(&self.high.clone().unwrap())
                    .unwrap()
                    .give(self.values[1]);
            } else {
                containers
                    .get_mut(&self.low.clone().unwrap())
                    .unwrap()
                    .give(self.values[1]);
                containers
                    .get_mut(&self.high.clone().unwrap())
                    .unwrap()
                    .give(self.values[0]);
            }
            return true;
        }
        false
    }
}

fn main() {
    let input = include_str!("../input");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let mut containers: HashMap<String, Container> = HashMap::new();

    for line in input
        .lines()
        .map(|l| l.split_whitespace().collect::<Vec<&str>>())
    {
        if line[0] == "value" {
            containers
                .entry(format!("bot {}", line[5]))
                .and_modify(|b| b.give(line[1].parse::<usize>().unwrap()))
                .or_insert(Container {
                    is_bot: true,
                    values: vec![line[1].parse().unwrap()],
                    ..Default::default()
                });
        } else {
            containers
                .entry(format!("{} {}", line[5], line[6]))
                .or_insert(Container {
                    is_bot: line[5] == "bot",
                    ..Default::default()
                });
            containers
                .entry(format!("{} {}", line[10], line[11]))
                .or_insert(Container {
                    is_bot: line[10] == "bot",
                    ..Default::default()
                });

            containers
                .entry(format!("bot {}", line[1]))
                .and_modify(|b| {
                    b.low = Some(format!("{} {}", line[5], line[6]));
                    b.high = Some(format!("{} {}", line[10], line[11]));
                })
                .or_insert(Container {
                    is_bot: true,
                    low: Some(format!("{} {}", line[5], line[6])),
                    high: Some(format!("{} {}", line[10], line[11])),
                    ..Default::default()
                });
        }
    }

    loop {
        let mut to_execute = containers
            .iter()
            .filter_map(|c| {
                if c.1.is_bot && !c.1.executed {
                    Some((c.0.clone(), c.1.clone(), false))
                } else {
                    None
                }
            })
            .collect::<Vec<(String, Container, bool)>>();

        if to_execute.is_empty() {
            break;
        }

        for b in to_execute.iter_mut() {
            b.2 = b.1.execute(&mut containers);
            containers
                .entry(b.0.clone())
                .and_modify(|c| c.executed |= b.2);
        }
    }

    containers
        .iter()
        .filter_map(|c| {
            if c.1.is_bot && c.1.values.contains(&61) && c.1.values.contains(&17) {
                Some(
                    c.0.split_whitespace()
                        .last()
                        .unwrap()
                        .parse::<usize>()
                        .unwrap(),
                )
            } else {
                None
            }
        })
        .next()
        .unwrap()
}

fn part2(input: &str) -> usize {
    let mut containers: HashMap<String, Container> = HashMap::new();

    for line in input
        .lines()
        .map(|l| l.split_whitespace().collect::<Vec<&str>>())
    {
        if line[0] == "value" {
            containers
                .entry(format!("bot {}", line[5]))
                .and_modify(|b| b.give(line[1].parse::<usize>().unwrap()))
                .or_insert(Container {
                    is_bot: true,
                    values: vec![line[1].parse().unwrap()],
                    ..Default::default()
                });
        } else {
            containers
                .entry(format!("{} {}", line[5], line[6]))
                .or_insert(Container {
                    is_bot: line[5] == "bot",
                    ..Default::default()
                });
            containers
                .entry(format!("{} {}", line[10], line[11]))
                .or_insert(Container {
                    is_bot: line[10] == "bot",
                    ..Default::default()
                });

            containers
                .entry(format!("bot {}", line[1]))
                .and_modify(|b| {
                    b.low = Some(format!("{} {}", line[5], line[6]));
                    b.high = Some(format!("{} {}", line[10], line[11]));
                })
                .or_insert(Container {
                    is_bot: true,
                    low: Some(format!("{} {}", line[5], line[6])),
                    high: Some(format!("{} {}", line[10], line[11])),
                    ..Default::default()
                });
        }
    }

    loop {
        let mut to_execute = containers
            .iter()
            .filter_map(|c| {
                if c.1.is_bot && !c.1.executed {
                    Some((c.0.clone(), c.1.clone(), false))
                } else {
                    None
                }
            })
            .collect::<Vec<(String, Container, bool)>>();

        if to_execute.is_empty() {
            break;
        }

        for b in to_execute.iter_mut() {
            b.2 = b.1.execute(&mut containers);
            containers
                .entry(b.0.clone())
                .and_modify(|c| c.executed |= b.2);
        }
    }

    containers
        .iter()
        .filter_map(|c| {
            if c.0 == "output 0" || c.0 == "output 1" || *c.0 == "output 2" {
                Some(c.1.values[0])
            } else {
                None
            }
        })
        .product()
}
