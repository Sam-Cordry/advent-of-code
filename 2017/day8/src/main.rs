use std::collections::HashMap;

fn main() {
    let input = include_str!("../input");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> i32 {
    let mut registers: HashMap<&str, i32> = HashMap::new();

    for instr in input
        .lines()
        .map(|l| l.split_whitespace().collect::<Vec<&str>>())
    {
        if !registers.contains_key(instr[4]) {
            registers.insert(instr[4], 0);
        }

        if match instr[5] {
            ">" => registers[instr[4]] > instr[6].parse::<i32>().unwrap(),
            "<" => registers[instr[4]] < instr[6].parse::<i32>().unwrap(),
            ">=" => registers[instr[4]] >= instr[6].parse::<i32>().unwrap(),
            "<=" => registers[instr[4]] <= instr[6].parse::<i32>().unwrap(),
            "==" => registers[instr[4]] == instr[6].parse::<i32>().unwrap(),
            "!=" => registers[instr[4]] != instr[6].parse::<i32>().unwrap(),
            _ => panic!("invalid instruction"),
        } {
            registers
                .entry(instr[0])
                .and_modify(|v| {
                    *v += instr[2].parse::<i32>().unwrap() * if instr[1] == "inc" { 1 } else { -1 }
                })
                .or_insert(
                    instr[2].parse::<i32>().unwrap() * if instr[1] == "inc" { 1 } else { -1 },
                );
        }
    }

    *registers.values().max().unwrap()
}

fn part2(input: &str) -> i32 {
    let mut registers: HashMap<&str, i32> = HashMap::new();
    let mut max = i32::MIN;

    for instr in input
        .lines()
        .map(|l| l.split_whitespace().collect::<Vec<&str>>())
    {
        if !registers.contains_key(instr[4]) {
            registers.insert(instr[4], 0);
        }

        if match instr[5] {
            ">" => registers[instr[4]] > instr[6].parse::<i32>().unwrap(),
            "<" => registers[instr[4]] < instr[6].parse::<i32>().unwrap(),
            ">=" => registers[instr[4]] >= instr[6].parse::<i32>().unwrap(),
            "<=" => registers[instr[4]] <= instr[6].parse::<i32>().unwrap(),
            "==" => registers[instr[4]] == instr[6].parse::<i32>().unwrap(),
            "!=" => registers[instr[4]] != instr[6].parse::<i32>().unwrap(),
            _ => panic!("invalid instruction"),
        } {
            registers
                .entry(instr[0])
                .and_modify(|v| {
                    *v += instr[2].parse::<i32>().unwrap() * if instr[1] == "inc" { 1 } else { -1 }
                })
                .or_insert(
                    instr[2].parse::<i32>().unwrap() * if instr[1] == "inc" { 1 } else { -1 },
                );
        }

        max = max.max(*registers.values().max().unwrap());
    }

    max
}
