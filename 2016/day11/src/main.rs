use itertools::Itertools;
use std::{
    collections::{HashSet, VecDeque},
    hash::Hash,
};

#[derive(PartialEq, Eq, Clone, Hash, Debug)]
struct State {
    elevator: usize,
    floors: Vec<Vec<String>>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            elevator: 1,
            floors: vec![vec![], vec![], vec![], vec![]],
        }
    }
}

impl State {
    fn get_next(&self) -> Vec<Self> {
        let mut result = vec![];

        for i in 1..=4 {
            if self.elevator.abs_diff(i) == 1 {
                let mut poss = self.floors[self.elevator - 1]
                    .clone()
                    .iter()
                    .map(|i| vec![i.clone()])
                    .collect::<Vec<Vec<String>>>();
                poss.extend(
                    self.floors[self.elevator - 1]
                        .iter()
                        .permutations(2)
                        .map(|p| {
                            let mut new = vec![p[0].clone(), p[1].clone()];
                            new.sort();
                            new
                        })
                        .unique()
                        .collect::<Vec<Vec<String>>>(),
                );

                for p in poss {
                    let mut new_floor = self.floors[i - 1].clone();
                    new_floor.append(&mut p.clone());
                    new_floor.sort();

                    let mut old_floor = self.floors[self.elevator - 1].clone();
                    for i in p.iter() {
                        if let Some(idx) = old_floor.iter().position(|o| o == i) {
                            old_floor.remove(idx);
                        }
                    }

                    if validate_floor(&new_floor) && validate_floor(&old_floor) {
                        let mut new = self.floors.clone();

                        new[self.elevator - 1] = old_floor;
                        new[i - 1] = new_floor;

                        result.push(State {
                            elevator: i,
                            floors: new,
                        });
                    }
                }
            }
        }

        result
    }

    fn is_goal(&self) -> bool {
        self.floors[0].is_empty() && self.floors[1].is_empty() && self.floors[2].is_empty()
    }
}

fn validate_floor(floor: &[String]) -> bool {
    for item in floor {
        if item.contains("microchip")
            && !floor
                .iter()
                .any(|i| *i == format!("{} generator", item.split('-').next().unwrap()))
            && floor.iter().any(|i| i.contains("generator"))
        {
            return false;
        }
    }

    true
}

fn main() {
    let input = include_str!("../input");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let mut initial = State::default();

    for (idx, line) in input
        .lines()
        .map(|l| l.split_whitespace().collect::<Vec<&str>>())
        .enumerate()
    {
        for (i, l) in line.iter().enumerate() {
            if *l == "a" {
                initial.floors[idx].push(if let Some(end) = line[i + 2].strip_suffix([',', '.']) {
                    format!("{} {}", line[i + 1], end)
                } else {
                    format!("{} {}", line[i + 1], line[i + 2])
                });
            }
        }
        initial.floors[idx].sort();
    }

    let mut queue: VecDeque<(usize, State)> = VecDeque::new();
    queue.push_back((0, initial.clone()));

    let mut seen: HashSet<State> = HashSet::new();
    seen.insert(initial);

    while let Some((steps, current)) = queue.pop_front() {
        for next in current.get_next() {
            if next.is_goal() {
                return steps + 1;
            } else if !seen.contains(&next) {
                seen.insert(next.clone());
                queue.push_back((steps + 1, next));
            }
        }
    }

    0
}

fn part2(input: &str) -> usize {
    let mut initial = State::default();

    initial.floors[0].push("elerium generator".to_string());
    initial.floors[0].push("elerium-compatible microchip".to_string());
    initial.floors[0].push("dilithium generator".to_string());
    initial.floors[0].push("dilithium-compatible microchip".to_string());

    for (idx, line) in input
        .lines()
        .map(|l| l.split_whitespace().collect::<Vec<&str>>())
        .enumerate()
    {
        for (i, l) in line.iter().enumerate() {
            if *l == "a" {
                initial.floors[idx].push(if let Some(end) = line[i + 2].strip_suffix([',', '.']) {
                    format!("{} {}", line[i + 1], end)
                } else {
                    format!("{} {}", line[i + 1], line[i + 2])
                });
            }
        }
        initial.floors[idx].sort();
    }

    let mut queue: VecDeque<(usize, State)> = VecDeque::new();
    queue.push_back((0, initial.clone()));

    let mut seen: HashSet<State> = HashSet::new();
    seen.insert(initial);

    while let Some((steps, current)) = queue.pop_front() {
        for next in current.get_next() {
            if next.is_goal() {
                return steps + 1;
            } else if !seen.contains(&next) {
                seen.insert(next.clone());
                queue.push_back((steps + 1, next));
            }
        }
    }

    0
}
