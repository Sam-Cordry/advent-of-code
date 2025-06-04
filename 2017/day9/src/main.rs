fn main() {
    let input = include_str!("../input");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let mut result = 0;
    let mut depth = 0;
    let mut idx = 0;
    let mut garbage = false;

    while idx < input.trim().len() {
        match (garbage, input.chars().nth(idx).unwrap()) {
            (true, '>') => {
                garbage = false;
            }
            (true, '!') => {
                idx += 1;
            }
            (false, '{') => {
                depth += 1;
                result += depth;
            }
            (false, '}') => {
                depth -= 1;
            }
            (false, '<') => {
                garbage = true;
            }
            _ => {}
        }
        idx += 1;
    }

    result
}

fn part2(input: &str) -> usize {
    let mut result = 0;
    let mut idx = 0;
    let mut garbage = false;

    while idx < input.trim().len() {
        match (garbage, input.chars().nth(idx).unwrap()) {
            (true, '>') => {
                garbage = false;
            }
            (true, '!') => {
                idx += 1;
            }
            (true, _) => {
                result += 1;
            }
            (false, '<') => {
                garbage = true;
            }
            _ => {}
        }
        idx += 1;
    }

    result
}
