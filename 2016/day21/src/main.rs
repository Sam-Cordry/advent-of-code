fn main() {
    let input = include_str!("../input");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn swap(s: &mut String, a: usize, b: usize) {
    let mut new = String::new();

    new.push_str(&s.chars().take(a).collect::<String>());
    new.push(s.chars().nth(b).unwrap());
    new.push_str(&s.chars().take(b).skip(a + 1).collect::<String>());
    new.push(s.chars().nth(a).unwrap());
    new.push_str(&s.chars().skip(b + 1).collect::<String>());

    *s = new;
}

fn rotate(s: &mut String, right: bool, mut steps: usize) {
    steps %= s.len();

    let mut new = s.split_off(if right { s.len() - steps } else { steps });
    new.push_str(s);

    *s = new;
}

fn reverse(s: &mut String, a: usize, b: usize) {
    let mut new = String::new();

    new.push_str(&s.chars().take(a).collect::<String>());
    new.push_str(
        &s.chars()
            .take(b + 1)
            .skip(a)
            .collect::<Vec<char>>()
            .iter()
            .rev()
            .collect::<String>(),
    );
    new.push_str(&s.chars().skip(b + 1).collect::<String>());

    *s = new;
}

fn move_letter(s: &mut String, from: usize, to: usize) {
    let mut new = String::new();

    if from < to {
        new.push_str(&s.chars().take(from).collect::<String>());
        new.push_str(&s.chars().take(to + 1).skip(from + 1).collect::<String>());
        new.push(s.chars().nth(from).unwrap());
        new.push_str(&s.chars().skip(to + 1).collect::<String>());
    } else {
        new.push_str(&s.chars().take(to).collect::<String>());
        new.push(s.chars().nth(from).unwrap());
        new.push_str(&s.chars().take(from).skip(to).collect::<String>());
        new.push_str(&s.chars().skip(from + 1).collect::<String>());
    }

    *s = new;
}

fn part1(input: &str) -> String {
    let mut current = String::from("abcdefgh");

    for instr in input
        .lines()
        .map(|l| l.split_whitespace().collect::<Vec<&str>>())
    {
        match (instr[0], instr[1]) {
            ("swap", "position") => {
                let a = instr[2].parse::<usize>().unwrap();
                let b = instr[5].parse::<usize>().unwrap();

                if a < b {
                    swap(&mut current, a, b);
                } else {
                    swap(&mut current, b, a);
                }
            }
            ("swap", "letter") => {
                let a = current
                    .chars()
                    .position(|c| c == instr[2].chars().next().unwrap())
                    .unwrap();
                let b = current
                    .chars()
                    .position(|c| c == instr[5].chars().next().unwrap())
                    .unwrap();

                if a < b {
                    swap(&mut current, a, b);
                } else {
                    swap(&mut current, b, a);
                }
            }
            ("rotate", "left") => {
                rotate(&mut current, false, instr[2].parse::<usize>().unwrap());
            }
            ("rotate", "right") => {
                rotate(&mut current, true, instr[2].parse::<usize>().unwrap());
            }
            ("rotate", "based") => {
                let mut n = current
                    .chars()
                    .position(|c| c == instr[6].chars().next().unwrap())
                    .unwrap();

                if n >= 4 {
                    n += 1;
                }

                rotate(&mut current, true, n + 1);
            }
            ("reverse", "positions") => {
                reverse(
                    &mut current,
                    instr[2].parse::<usize>().unwrap(),
                    instr[4].parse::<usize>().unwrap(),
                );
            }
            ("move", "position") => {
                move_letter(
                    &mut current,
                    instr[2].parse::<usize>().unwrap(),
                    instr[5].parse::<usize>().unwrap(),
                );
            }
            _ => unreachable!("invalid instruction"),
        }
    }

    current
}

fn part2(input: &str) -> String {
    let mut current = String::from("fbgdceah");

    for instr in input
        .lines()
        .map(|l| l.split_whitespace().collect::<Vec<&str>>())
        .rev()
    {
        match (instr[0], instr[1]) {
            ("swap", "position") => {
                let a = instr[2].parse::<usize>().unwrap();
                let b = instr[5].parse::<usize>().unwrap();

                if a < b {
                    swap(&mut current, a, b);
                } else {
                    swap(&mut current, b, a);
                }
            }
            ("swap", "letter") => {
                let a = current
                    .chars()
                    .position(|c| c == instr[2].chars().next().unwrap())
                    .unwrap();
                let b = current
                    .chars()
                    .position(|c| c == instr[5].chars().next().unwrap())
                    .unwrap();

                if a < b {
                    swap(&mut current, a, b);
                } else {
                    swap(&mut current, b, a);
                }
            }
            ("rotate", "left") => {
                rotate(&mut current, true, instr[2].parse::<usize>().unwrap());
            }
            ("rotate", "right") => {
                rotate(&mut current, false, instr[2].parse::<usize>().unwrap());
            }
            ("rotate", "based") => {
                let end = current
                    .chars()
                    .position(|c| c == instr[6].chars().next().unwrap())
                    .unwrap();

                loop {
                    rotate(&mut current, false, 1);

                    let start = current
                        .chars()
                        .position(|c| c == instr[6].chars().next().unwrap())
                        .unwrap();

                    if start >= 4 && (start * 2 + 2) % current.len() == end
                        || start < 4 && (start * 2 + 1) % current.len() == end
                    {
                        break;
                    }
                }
            }
            ("reverse", "positions") => {
                reverse(
                    &mut current,
                    instr[2].parse::<usize>().unwrap(),
                    instr[4].parse::<usize>().unwrap(),
                );
            }
            ("move", "position") => {
                move_letter(
                    &mut current,
                    instr[5].parse::<usize>().unwrap(),
                    instr[2].parse::<usize>().unwrap(),
                );
            }
            _ => unreachable!("invaild instruction"),
        }
    }

    current
}
