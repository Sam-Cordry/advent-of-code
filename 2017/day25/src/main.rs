use std::collections::HashMap;

type State = Box<dyn FnMut(&mut HashMap<i32, bool>, &mut i32) -> char>;

fn main() {
    let input = include_str!("../input");
    println!("Part 1: {}", part1(input));
}

fn part1(input: &'static str) -> usize {
    let lines: Vec<Vec<&str>> = input
        .lines()
        .map(|l| l.split_whitespace().collect())
        .collect();
    let mut states: HashMap<char, State> = HashMap::new();

    for i in (0..lines.len()).filter(|n| n % 10 == 3) {
        let info: Vec<Vec<&str>> = lines.iter().skip(i).take(9).cloned().collect();
        states.insert(
            info[0].last().unwrap().chars().next().unwrap(),
            Box::from(move |tape: &mut HashMap<i32, bool>, idx: &mut i32| {
                if !tape.get(idx).copied().unwrap_or_default() {
                    let write = info[2].last().unwrap().starts_with('1');
                    tape.entry(*idx).and_modify(|v| *v = write).or_insert(write);
                    *idx += if info[3].last().unwrap() == &"right." {
                        1
                    } else {
                        -1
                    };
                    info[4].last().unwrap().chars().next().unwrap()
                } else {
                    let write = info[6].last().unwrap().starts_with('1');
                    tape.entry(*idx).and_modify(|v| *v = write).or_insert(write);
                    *idx += if info[7].last().unwrap() == &"right." {
                        1
                    } else {
                        -1
                    };
                    info[8].last().unwrap().chars().next().unwrap()
                }
            }),
        );
    }

    let mut tape: HashMap<i32, bool> = HashMap::new();
    let mut current = 'A';
    let mut idx = 0;

    for _ in 0..lines[1][lines[1].len() - 2].parse::<usize>().unwrap() {
        current = states.get_mut(&current).unwrap()(&mut tape, &mut idx);
    }

    tape.iter().filter(|e| *e.1).count()
}
