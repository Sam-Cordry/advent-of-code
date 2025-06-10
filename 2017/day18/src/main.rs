use std::collections::{HashMap, VecDeque};

fn main() {
    let input = include_str!("../input");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn sound_based<'a>(
    instr: &[&'a str],
    i: &mut usize,
    registers: &mut HashMap<&'a str, i64>,
    last_played: &mut i64,
) -> Option<i64> {
    match instr[0] {
        "snd" => {
            if let Ok(x) = instr[1].parse::<i64>() {
                *last_played = x;
            } else {
                if !registers.contains_key(instr[1]) {
                    registers.insert(instr[1], 0);
                }

                *last_played = registers[instr[1]];
            }

            *i += 1;

            None
        }
        "set" => {
            let y = if let Ok(y) = instr[2].parse::<i64>() {
                y
            } else {
                if !registers.contains_key(instr[2]) {
                    registers.insert(instr[2], 0);
                }

                registers[instr[2]]
            };

            registers.insert(instr[1], y);
            *i += 1;

            None
        }
        "add" => {
            let y = if let Ok(y) = instr[2].parse::<i64>() {
                y
            } else {
                if !registers.contains_key(instr[2]) {
                    registers.insert(instr[2], 0);
                }

                registers[instr[2]]
            };

            registers
                .entry(instr[1])
                .and_modify(|v| *v += y)
                .or_insert(y);
            *i += 1;

            None
        }
        "mul" => {
            let y = if let Ok(y) = instr[2].parse::<i64>() {
                y
            } else {
                if !registers.contains_key(instr[2]) {
                    registers.insert(instr[2], 0);
                }

                registers[instr[2]]
            };

            registers
                .entry(instr[1])
                .and_modify(|v| *v *= y)
                .or_insert(0);
            *i += 1;

            None
        }
        "mod" => {
            let y = if let Ok(y) = instr[2].parse::<i64>() {
                y
            } else {
                if !registers.contains_key(instr[2]) {
                    registers.insert(instr[2], 0);
                }

                registers[instr[2]]
            };

            registers
                .entry(instr[1])
                .and_modify(|v| *v %= y)
                .or_insert(0);
            *i += 1;

            None
        }
        "rcv" => {
            let x = if let Ok(x) = instr[1].parse::<i64>() {
                x
            } else {
                if !registers.contains_key(instr[1]) {
                    registers.insert(instr[1], 0);
                }

                registers[instr[1]]
            };

            if x != 0 {
                return Some(*last_played);
            }
            *i += 1;

            None
        }
        "jgz" => {
            let x = if let Ok(x) = instr[1].parse::<i64>() {
                x
            } else {
                if !registers.contains_key(instr[1]) {
                    registers.insert(instr[1], 0);
                }

                registers[instr[1]]
            };

            if x > 0 {
                let y = if let Ok(y) = instr[2].parse::<i64>() {
                    y
                } else {
                    if !registers.contains_key(instr[2]) {
                        registers.insert(instr[2], 0);
                    }

                    registers[instr[2]]
                };

                *i = (*i as i64 + y) as usize;
            } else {
                *i += 1;
            }

            None
        }
        _ => unreachable!("invalid instruction"),
    }
}

fn part1(input: &str) -> i64 {
    let instructions = input
        .lines()
        .map(|l| l.split_whitespace().collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();
    let mut registers: HashMap<&str, i64> = HashMap::new();
    let mut last_played: i64 = 0;
    let mut i = 0;

    while i < instructions.len() {
        if let Some(result) =
            sound_based(&instructions[i], &mut i, &mut registers, &mut last_played)
        {
            return result;
        }
    }

    0
}

fn concurrent_based<'a>(
    instr: &[&'a str],
    i: &mut usize,
    registers: &mut HashMap<&'a str, i64>,
    messages: &mut VecDeque<i64>,
    other_messages: &mut VecDeque<i64>,
    waiting: &mut bool,
) -> usize {
    match instr[0] {
        "snd" => {
            if let Ok(x) = instr[1].parse::<i64>() {
                other_messages.push_back(x);
            } else {
                other_messages.push_back(registers[&instr[1]]);
            }

            *i += 1;

            1
        }
        "set" => {
            let y = if let Ok(y) = instr[2].parse::<i64>() {
                y
            } else {
                if !registers.contains_key(instr[2]) {
                    registers.insert(instr[2], 0);
                }

                registers[instr[2]]
            };

            registers.insert(instr[1], y);
            *i += 1;

            0
        }
        "add" => {
            let y = if let Ok(y) = instr[2].parse::<i64>() {
                y
            } else {
                if !registers.contains_key(instr[2]) {
                    registers.insert(instr[2], 0);
                }

                registers[instr[2]]
            };

            registers
                .entry(instr[1])
                .and_modify(|v| *v += y)
                .or_insert(y);
            *i += 1;

            0
        }
        "mul" => {
            let y = if let Ok(y) = instr[2].parse::<i64>() {
                y
            } else {
                if !registers.contains_key(instr[2]) {
                    registers.insert(instr[2], 0);
                }

                registers[instr[2]]
            };

            registers
                .entry(instr[1])
                .and_modify(|v| *v *= y)
                .or_insert(0);
            *i += 1;

            0
        }
        "mod" => {
            let y = if let Ok(y) = instr[2].parse::<i64>() {
                y
            } else {
                if !registers.contains_key(instr[2]) {
                    registers.insert(instr[2], 0);
                }

                registers[instr[2]]
            };

            registers
                .entry(instr[1])
                .and_modify(|v| *v %= y)
                .or_insert(0);
            *i += 1;

            0
        }
        "rcv" => {
            if messages.is_empty() {
                *waiting = true;
            } else {
                *waiting = false;
                let message = messages.pop_front().unwrap();
                registers
                    .entry(instr[1])
                    .and_modify(|v| *v = message)
                    .or_insert(message);

                *i += 1;
            }

            0
        }
        "jgz" => {
            let x = if let Ok(x) = instr[1].parse::<i64>() {
                x
            } else {
                if !registers.contains_key(instr[1]) {
                    registers.insert(instr[1], 0);
                }

                registers[instr[1]]
            };

            if x > 0 {
                let y = if let Ok(y) = instr[2].parse::<i64>() {
                    y
                } else {
                    if !registers.contains_key(instr[2]) {
                        registers.insert(instr[2], 0);
                    }

                    registers[instr[2]]
                };

                *i = (*i as i64 + y) as usize;
            } else {
                *i += 1;
            }

            0
        }
        _ => unreachable!("invalid instruction"),
    }
}

fn part2(input: &str) -> usize {
    let instructions = input
        .lines()
        .map(|l| l.split_whitespace().collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    let mut registers_a: HashMap<&str, i64> = HashMap::from_iter([("p", 0)]);
    let mut messages_a: VecDeque<i64> = VecDeque::new();
    let mut waiting_a = false;
    let mut idx_a: usize = 0;

    let mut registers_b: HashMap<&str, i64> = HashMap::from_iter([("p", 1)]);
    let mut messages_b: VecDeque<i64> = VecDeque::new();
    let mut waiting_b = false;
    let mut idx_b: usize = 0;

    let mut result = 0;

    while (idx_a < instructions.len() && idx_b < instructions.len())
        && !(waiting_a && messages_a.is_empty() && waiting_b && messages_b.is_empty())
    {
        while idx_a < instructions.len() && !(waiting_a && messages_a.is_empty()) {
            concurrent_based(
                &instructions[idx_a],
                &mut idx_a,
                &mut registers_a,
                &mut messages_a,
                &mut messages_b,
                &mut waiting_a,
            );
        }

        while idx_b < instructions.len() && !(waiting_b && messages_b.is_empty()) {
            result += concurrent_based(
                &instructions[idx_b],
                &mut idx_b,
                &mut registers_b,
                &mut messages_b,
                &mut messages_a,
                &mut waiting_b,
            );
        }
    }

    result
}
