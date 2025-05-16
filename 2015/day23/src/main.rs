use std::collections::HashMap;

fn main() {
    let input = include_str!("../input");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let instructions = input
        .lines()
        .map(|l| l.split_whitespace().collect())
        .collect::<Vec<Vec<&str>>>();
    let mut registers: HashMap<char, usize> = HashMap::from([('a', 0), ('b', 0)]);
    let mut pc: usize = 0;

    while pc < instructions.len() {
        match instructions[pc][0] {
            "hlf" => {
                registers.insert(
                    instructions[pc][1].chars().next().unwrap(),
                    registers[&instructions[pc][1].chars().next().unwrap()] / 2,
                );
            }
            "tpl" => {
                registers.insert(
                    instructions[pc][1].chars().next().unwrap(),
                    registers[&instructions[pc][1].chars().next().unwrap()] * 3,
                );
            }
            "inc" => {
                registers.insert(
                    instructions[pc][1].chars().next().unwrap(),
                    registers[&instructions[pc][1].chars().next().unwrap()] + 1,
                );
            }
            "jmp" => {
                if instructions[pc][1].starts_with('+') {
                    pc += instructions[pc][1][1..].parse::<usize>().unwrap();
                } else {
                    pc -= instructions[pc][1][1..].parse::<usize>().unwrap();
                }
                continue;
            }
            "jie" => {
                if registers[&instructions[pc][1].chars().next().unwrap()] % 2 == 0 {
                    if instructions[pc][2].starts_with('+') {
                        pc += instructions[pc][2][1..].parse::<usize>().unwrap();
                    } else {
                        pc -= instructions[pc][2][1..].parse::<usize>().unwrap();
                    }
                    continue;
                }
            }
            "jio" => {
                if registers[&instructions[pc][1].chars().next().unwrap()] == 1 {
                    if instructions[pc][2].starts_with('+') {
                        pc += instructions[pc][2][1..].parse::<usize>().unwrap();
                    } else {
                        pc -= instructions[pc][2][1..].parse::<usize>().unwrap();
                    }
                    continue;
                }
            }
            _ => panic!("invalid instruction"),
        }
        pc += 1;
    }

    registers[&'b']
}

fn part2(input: &str) -> usize {
    let instructions = input
        .lines()
        .map(|l| l.split_whitespace().collect())
        .collect::<Vec<Vec<&str>>>();
    let mut registers: HashMap<char, usize> = HashMap::from([('a', 1), ('b', 0)]);
    let mut pc: usize = 0;

    while pc < instructions.len() {
        match instructions[pc][0] {
            "hlf" => {
                registers.insert(
                    instructions[pc][1].chars().next().unwrap(),
                    registers[&instructions[pc][1].chars().next().unwrap()] / 2,
                );
            }
            "tpl" => {
                registers.insert(
                    instructions[pc][1].chars().next().unwrap(),
                    registers[&instructions[pc][1].chars().next().unwrap()] * 3,
                );
            }
            "inc" => {
                registers.insert(
                    instructions[pc][1].chars().next().unwrap(),
                    registers[&instructions[pc][1].chars().next().unwrap()] + 1,
                );
            }
            "jmp" => {
                if instructions[pc][1].starts_with('+') {
                    pc += instructions[pc][1][1..].parse::<usize>().unwrap();
                } else {
                    pc -= instructions[pc][1][1..].parse::<usize>().unwrap();
                }
                continue;
            }
            "jie" => {
                if registers[&instructions[pc][1].chars().next().unwrap()] % 2 == 0 {
                    if instructions[pc][2].starts_with('+') {
                        pc += instructions[pc][2][1..].parse::<usize>().unwrap();
                    } else {
                        pc -= instructions[pc][2][1..].parse::<usize>().unwrap();
                    }
                    continue;
                }
            }
            "jio" => {
                if registers[&instructions[pc][1].chars().next().unwrap()] == 1 {
                    if instructions[pc][2].starts_with('+') {
                        pc += instructions[pc][2][1..].parse::<usize>().unwrap();
                    } else {
                        pc -= instructions[pc][2][1..].parse::<usize>().unwrap();
                    }
                    continue;
                }
            }
            _ => panic!("invalid instruction"),
        }
        pc += 1;
    }

    registers[&'b']
}
