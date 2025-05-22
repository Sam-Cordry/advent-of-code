use std::collections::HashMap;

fn main() {
    let input = include_str!("../input");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> i32 {
    let mut registers: HashMap<&str, i32> = HashMap::from([("a", 0), ("b", 0), ("c", 0), ("d", 0)]);

    let instructions = input
        .lines()
        .map(|l| l.split_whitespace().collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();
    let mut idx = 0;

    while idx < instructions.len() {
        match instructions[idx][0] {
            "cpy" => {
                if let Ok(n) = instructions[idx][1].parse::<i32>() {
                    registers.entry(instructions[idx][2]).and_modify(|r| *r = n);
                } else {
                    let temp = registers[&instructions[idx][1]];
                    registers
                        .entry(instructions[idx][2])
                        .and_modify(|r| *r = temp);
                }
                idx += 1;
            }
            "inc" => {
                registers
                    .entry(instructions[idx][1])
                    .and_modify(|r| *r += 1);
                idx += 1;
            }
            "dec" => {
                registers
                    .entry(instructions[idx][1])
                    .and_modify(|r| *r -= 1);
                idx += 1;
            }
            "jnz" => {
                if let Ok(x) = instructions[idx][1].parse::<i32>() {
                    if x != 0 {
                        if instructions[idx][2].starts_with("-") {
                            idx -= instructions[idx][2][1..].parse::<usize>().unwrap();
                        } else {
                            idx += instructions[idx][2].parse::<usize>().unwrap();
                        }
                    } else {
                        idx += 1;
                    }
                } else if registers[&instructions[idx][1]] != 0 {
                    if instructions[idx][2].starts_with("-") {
                        idx -= instructions[idx][2][1..].parse::<usize>().unwrap();
                    } else {
                        idx += instructions[idx][2].parse::<usize>().unwrap();
                    }
                } else {
                    idx += 1;
                }
            }
            _ => panic!("invalid instruction"),
        }
    }

    registers[&"a"]
}

fn part2(input: &str) -> i32 {
    let mut registers: HashMap<&str, i32> = HashMap::from([("a", 0), ("b", 0), ("c", 1), ("d", 0)]);

    let instructions = input
        .lines()
        .map(|l| l.split_whitespace().collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();
    let mut idx = 0;

    while idx < instructions.len() {
        match instructions[idx][0] {
            "cpy" => {
                if let Ok(n) = instructions[idx][1].parse::<i32>() {
                    registers.entry(instructions[idx][2]).and_modify(|r| *r = n);
                } else {
                    let temp = registers[&instructions[idx][1]];
                    registers
                        .entry(instructions[idx][2])
                        .and_modify(|r| *r = temp);
                }
                idx += 1;
            }
            "inc" => {
                registers
                    .entry(instructions[idx][1])
                    .and_modify(|r| *r += 1);
                idx += 1;
            }
            "dec" => {
                registers
                    .entry(instructions[idx][1])
                    .and_modify(|r| *r -= 1);
                idx += 1;
            }
            "jnz" => {
                if let Ok(x) = instructions[idx][1].parse::<i32>() {
                    if x != 0 {
                        if instructions[idx][2].starts_with("-") {
                            idx -= instructions[idx][2][1..].parse::<usize>().unwrap();
                        } else {
                            idx += instructions[idx][2].parse::<usize>().unwrap();
                        }
                    } else {
                        idx += 1;
                    }
                } else if registers[&instructions[idx][1]] != 0 {
                    if instructions[idx][2].starts_with("-") {
                        idx -= instructions[idx][2][1..].parse::<usize>().unwrap();
                    } else {
                        idx += instructions[idx][2].parse::<usize>().unwrap();
                    }
                } else {
                    idx += 1;
                }
            }
            _ => panic!("invalid instruction"),
        }
    }

    registers[&"a"]
}
