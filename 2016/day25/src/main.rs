use std::collections::HashMap;

fn main() {
    let input = include_str!("../input");
    println!("Part 1: {}", part1(input));
}

fn part1(input: &str) -> usize {
    let mut instructions = input
        .lines()
        .map(|l| l.split_whitespace().collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();
    let mut count: usize = 1;

    loop {
        let mut idx = 0;
        let mut registers: HashMap<&str, i32> =
            HashMap::from([("a", count as i32), ("b", 0), ("c", 0), ("d", 0)]);
        let mut out: Vec<i32> = vec![];

        while idx < instructions.len() && out.len() < 50 {
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
                    if offset + (idx as i32) >= 0
                        && offset + (idx as i32) < instructions.len() as i32
                    {
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
                "out" => {
                    if let Ok(n) = instructions[idx][1].parse::<i32>() {
                        out.push(n);
                    } else {
                        out.push(registers[&instructions[idx][1]]);
                    }
                    idx += 1;
                }
                _ => panic!("invalid instruction"),
            }

            if out.len() >= 2
                && (out.iter().any(|&n| n != 0 && n != 1) || out.windows(2).any(|w| w[0] == w[1]))
            {
                break;
            }
        }

        if out.len() == 50
            && out.iter().all(|&n| n == 0 || n == 1)
            && out.windows(2).all(|w| w[0] != w[1])
        {
            return count;
        }

        count += 1;
    }
}
