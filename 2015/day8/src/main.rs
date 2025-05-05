fn main() {
    let input = include_str!("../input");

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let mut code: usize = 0;
    let mut memory: usize = 0;

    for l in input.lines().map(|l| l.trim()) {
        let mut iter = l.chars();
        while let Some(c) = iter.next() {
            if c == '"' {
                code += 1;
            } else if c == '\\' {
                let next = iter.next().unwrap();
                if next == 'x' {
                    iter.next();
                    iter.next();
                    code += 4;
                    memory += 1;
                } else if next == '"' || next == '\\' {
                    code += 2;
                    memory += 1;
                }
            } else {
                code += 1;
                memory += 1;
            }
        }
    }

    code - memory
}

fn part2(input: &str) -> usize {
    let mut code: usize = 0;
    let mut new: usize = 0;

    for l in input.lines().map(|l| l.trim()) {
        new += 2;
        for c in l.chars() {
            code += 1;
            if c == '"' || c == '\\' {
                new += 2;
            } else {
                new += 1;
            }
        }
    }

    new - code
}
