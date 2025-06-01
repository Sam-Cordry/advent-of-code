use std::collections::HashMap;

fn main() {
    let input = include_str!("../input");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> i32 {
    let mut registers: HashMap<&str, i32> = HashMap::from([("a", 7), ("b", 0), ("c", 0), ("d", 0)]);

    let mut instructions = input
        .lines()
        .map(|l| l.split_whitespace().collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();
    let mut idx = 0;

    while idx < instructions.len() {
        match instructions[idx][0] {
            "cpy" => {
                if instructions[idx][2].parse::<i32>().is_err() {
                    if let Ok(n) = instructions[idx][1].parse::<i32>() {
                        registers.entry(instructions[idx][2]).and_modify(|r| *r = n);
                    } else {
                        let temp = registers[&instructions[idx][1]];
                        registers
                            .entry(instructions[idx][2])
                            .and_modify(|r| *r = temp);
                    }
                }
                idx += 1;
            }
            "inc" => {
                if instructions[idx][1].parse::<i32>().is_err() {
                    registers
                        .entry(instructions[idx][1])
                        .and_modify(|r| *r += 1);
                }
                idx += 1;
            }
            "dec" => {
                if instructions[idx][1].parse::<i32>().is_err() {
                    registers
                        .entry(instructions[idx][1])
                        .and_modify(|r| *r -= 1);
                }
                idx += 1;
            }
            "jnz" => {
                match (
                    instructions[idx][1].parse::<i32>(),
                    instructions[idx][2].parse::<i32>(),
                ) {
                    (Ok(comp), Ok(jump)) => {
                        if comp != 0 {
                            if jump.is_positive() {
                                idx += jump as usize;
                            } else {
                                idx -= jump.unsigned_abs() as usize;
                            }
                        } else {
                            idx += 1;
                        }
                    }
                    (Ok(comp), Err(_)) => {
                        let jump = registers[&instructions[idx][2]];
                        if comp != 0 {
                            if jump.is_positive() {
                                idx += jump as usize;
                            } else {
                                idx -= jump.unsigned_abs() as usize;
                            }
                        } else {
                            idx += 1;
                        }
                    }
                    (Err(_), Ok(jump)) => {
                        let comp = registers[&instructions[idx][1]];
                        if comp != 0 {
                            if jump.is_positive() {
                                idx += jump as usize;
                            } else {
                                idx -= jump.unsigned_abs() as usize;
                            }
                        } else {
                            idx += 1;
                        }
                    }
                    (Err(_), Err(_)) => {
                        let comp = registers[&instructions[idx][1]];
                        let jump = registers[&instructions[idx][2]];
                        if comp != 0 {
                            if jump.is_positive() {
                                idx += jump as usize;
                            } else {
                                idx -= jump.unsigned_abs() as usize;
                            }
                        } else {
                            idx += 1;
                        }
                    }
                }
            }
            "tgl" => {
                let offset = registers[&instructions[idx][1]];
                if offset + (idx as i32) >= 0 && offset + (idx as i32) < instructions.len() as i32 {
                    let i = (offset + (idx as i32)) as usize;
                    if instructions[i].len() == 2 && instructions[i][0] != "inc" {
                        instructions[i][0] = "inc";
                    } else if instructions[i][0] == "inc" {
                        instructions[i][0] = "dec";
                    } else if instructions[i].len() == 3 && instructions[i][0] != "jnz" {
                        instructions[i][0] = "jnz";
                    } else if instructions[i][0] == "jnz" {
                        instructions[i][0] = "cpy";
                    }
                }
                idx += 1;
            }
            _ => panic!("invalid instruction"),
        }
    }

    registers[&"a"]
}

fn part2(input: &str) -> i32 {
    let mut registers: HashMap<&str, i32> =
        HashMap::from([("a", 12), ("b", 0), ("c", 0), ("d", 0)]);

    let mut instructions = input
        .lines()
        .map(|l| l.split_whitespace().collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();
    let mut idx = 0;

    while idx < instructions.len() {
        match instructions[idx][0] {
            "cpy" => {
                if instructions[idx][2].parse::<i32>().is_err() {
                    if let Ok(n) = instructions[idx][1].parse::<i32>() {
                        registers.entry(instructions[idx][2]).and_modify(|r| *r = n);
                    } else {
                        let temp = registers[&instructions[idx][1]];
                        registers
                            .entry(instructions[idx][2])
                            .and_modify(|r| *r = temp);
                    }
                }
                idx += 1;
            }
            "inc" => {
                if instructions[idx][1].parse::<i32>().is_err() {
                    registers
                        .entry(instructions[idx][1])
                        .and_modify(|r| *r += 1);
                }
                idx += 1;
            }
            "dec" => {
                if instructions[idx][1].parse::<i32>().is_err() {
                    registers
                        .entry(instructions[idx][1])
                        .and_modify(|r| *r -= 1);
                }
                idx += 1;
            }
            "jnz" => {
                match (
                    instructions[idx][1].parse::<i32>(),
                    instructions[idx][2].parse::<i32>(),
                ) {
                    (Ok(comp), Ok(jump)) => {
                        if comp != 0 {
                            if jump.is_positive() {
                                idx += jump as usize;
                            } else {
                                idx -= jump.unsigned_abs() as usize;
                            }
                        } else {
                            idx += 1;
                        }
                    }
                    (Ok(comp), Err(_)) => {
                        let jump = registers[&instructions[idx][2]];
                        if comp != 0 {
                            if jump.is_positive() {
                                idx += jump as usize;
                            } else {
                                idx -= jump.unsigned_abs() as usize;
                            }
                        } else {
                            idx += 1;
                        }
                    }
                    (Err(_), Ok(jump)) => {
                        let comp = registers[&instructions[idx][1]];
                        if comp != 0 {
                            if jump.is_positive() {
                                idx += jump as usize;
                            } else {
                                idx -= jump.unsigned_abs() as usize;
                            }
                        } else {
                            idx += 1;
                        }
                    }
                    (Err(_), Err(_)) => {
                        let comp = registers[&instructions[idx][1]];
                        let jump = registers[&instructions[idx][2]];
                        if comp != 0 {
                            if jump.is_positive() {
                                idx += jump as usize;
                            } else {
                                idx -= jump.unsigned_abs() as usize;
                            }
                        } else {
                            idx += 1;
                        }
                    }
                }
            }
            "tgl" => {
                let offset = registers[&instructions[idx][1]];
                if offset + (idx as i32) >= 0 && offset + (idx as i32) < instructions.len() as i32 {
                    let i = (offset + (idx as i32)) as usize;
                    if instructions[i].len() == 2 && instructions[i][0] != "inc" {
                        instructions[i][0] = "inc";
                    } else if instructions[i][0] == "inc" {
                        instructions[i][0] = "dec";
                    } else if instructions[i].len() == 3 && instructions[i][0] != "jnz" {
                        instructions[i][0] = "jnz";
                    } else if instructions[i][0] == "jnz" {
                        instructions[i][0] = "cpy";
                    }
                }
                idx += 1;
            }
            _ => panic!("invalid instruction"),
        }
    }

    registers[&"a"]
}
