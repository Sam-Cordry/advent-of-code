use std::collections::HashMap;

fn main() {
    let input = include_str!("../input");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let mut registers: HashMap<char, i32> = HashMap::from_iter(('a'..='h').map(|c| (c, 0)));

    let instructions: Vec<Vec<&str>> = input
        .lines()
        .map(|l| l.split_whitespace().collect())
        .collect();
    let mut i = 0;
    let mut result = 0;

    while i < instructions.len() {
        match instructions[i][0] {
            "set" => {
                if let Ok(y) = instructions[i][2].parse::<i32>() {
                    registers
                        .entry(instructions[i][1].chars().next().unwrap())
                        .and_modify(|v| *v = y);
                } else {
                    let temp = registers[&instructions[i][2].chars().next().unwrap()];
                    registers
                        .entry(instructions[i][1].chars().next().unwrap())
                        .and_modify(|v| *v = temp);
                }
                i += 1;
            }
            "sub" => {
                if let Ok(y) = instructions[i][2].parse::<i32>() {
                    registers
                        .entry(instructions[i][1].chars().next().unwrap())
                        .and_modify(|v| *v -= y);
                } else {
                    let temp = registers[&instructions[i][2].chars().next().unwrap()];
                    registers
                        .entry(instructions[i][1].chars().next().unwrap())
                        .and_modify(|v| *v -= temp);
                }
                i += 1;
            }
            "mul" => {
                if let Ok(y) = instructions[i][2].parse::<i32>() {
                    registers
                        .entry(instructions[i][1].chars().next().unwrap())
                        .and_modify(|v| *v *= y);
                } else {
                    let temp = registers[&instructions[i][2].chars().next().unwrap()];
                    registers
                        .entry(instructions[i][1].chars().next().unwrap())
                        .and_modify(|v| *v *= temp);
                }
                result += 1;
                i += 1;
            }
            "jnz" => {
                if instructions[i][1] != "0"
                    && registers
                        .get(&instructions[i][1].chars().next().unwrap())
                        .copied()
                        .unwrap_or(1)
                        != 0
                {
                    if let Ok(y) = instructions[i][2].parse::<i32>() {
                        i = (i as i32 + y) as usize;
                    } else {
                        i = (i as i32 + registers[&instructions[i][2].chars().next().unwrap()])
                            as usize;
                    }
                } else {
                    i += 1;
                }
            }
            _ => unreachable!("invalid instruction"),
        }
    }

    result
}

fn part2(_input: &str) -> i32 {
    let mut registers: HashMap<char, i32> =
        HashMap::from_iter([('b', 108400), ('c', 125400), ('d', 0), ('f', 0), ('h', 0)]);

    loop {
        registers.entry('f').and_modify(|v| *v = 1);
        registers.entry('d').and_modify(|v| *v = 2);
        while registers[&'d'] < registers[&'b'] {
            if registers[&'b'] % registers[&'d'] == 0 {
                registers.entry('f').and_modify(|v| *v = 0);
                break;
            }
            registers.entry('d').and_modify(|v| *v += 1);
        }
        if registers[&'f'] == 0 {
            registers.entry('h').and_modify(|v| *v += 1);
        }
        if registers[&'b'] - registers[&'c'] == 0 {
            break;
        }
        registers.entry('b').and_modify(|v| *v += 17);
    }

    registers[&'h']
}
