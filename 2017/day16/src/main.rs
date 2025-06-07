fn main() {
    let input = include_str!("../input");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn swap(s: &mut String, a: usize, b: usize) {
    let mut result = String::new();

    result.push_str(&s[..a]);
    result.push(s.chars().nth(b).unwrap());
    result.push_str(&s[a + 1..b]);
    result.push(s.chars().nth(a).unwrap());
    result.push_str(&s[b + 1..]);

    *s = result;
}

fn part1(input: &str) -> String {
    let mut order = String::from("abcdefghijklmnop");

    for instr in input.trim().split(',') {
        match instr.chars().next().unwrap() {
            's' => {
                let mut result =
                    order.split_off(order.len() - instr[1..].parse::<usize>().unwrap());
                result.push_str(&order);
                order = result;
            }
            'x' => {
                let a = instr[1..instr.find('/').unwrap()].parse::<usize>().unwrap();
                let b = instr[instr.find('/').unwrap() + 1..]
                    .parse::<usize>()
                    .unwrap();

                if a < b {
                    swap(&mut order, a, b);
                } else {
                    swap(&mut order, b, a);
                }
            }
            'p' => {
                let a = order
                    .find(instr[1..instr.find('/').unwrap()].chars().next().unwrap())
                    .unwrap();
                let b = order
                    .find(
                        instr[instr.find('/').unwrap() + 1..]
                            .chars()
                            .next()
                            .unwrap(),
                    )
                    .unwrap();

                if a < b {
                    swap(&mut order, a, b);
                } else {
                    swap(&mut order, b, a);
                }
            }
            _ => unreachable!("invalid instruction"),
        }
    }

    order
}

fn part2(input: &str) -> String {
    let mut order = String::from("abcdefghijklmnop");

    let mut i = 0;

    while i < 1000000000 {
        for instr in input.trim().split(',') {
            match instr.chars().next().unwrap() {
                's' => {
                    let mut result =
                        order.split_off(order.len() - instr[1..].parse::<usize>().unwrap());
                    result.push_str(&order);
                    order = result;
                }
                'x' => {
                    let a = instr[1..instr.find('/').unwrap()].parse::<usize>().unwrap();
                    let b = instr[instr.find('/').unwrap() + 1..]
                        .parse::<usize>()
                        .unwrap();

                    if a < b {
                        swap(&mut order, a, b);
                    } else {
                        swap(&mut order, b, a);
                    }
                }
                'p' => {
                    let a = order
                        .find(instr[1..instr.find('/').unwrap()].chars().next().unwrap())
                        .unwrap();
                    let b = order
                        .find(
                            instr[instr.find('/').unwrap() + 1..]
                                .chars()
                                .next()
                                .unwrap(),
                        )
                        .unwrap();

                    if a < b {
                        swap(&mut order, a, b);
                    } else {
                        swap(&mut order, b, a);
                    }
                }
                _ => unreachable!("invalid instruction"),
            }
        }

        if &order == "abcdefghijklmnop" {
            i = 1000000000 - (1000000000 % (i + 1));
        } else {
            i += 1;
        }
    }

    order
}
