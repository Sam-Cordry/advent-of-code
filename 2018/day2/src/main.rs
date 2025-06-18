use std::collections::HashMap;

fn main() {
    let input = include_str!("../input");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let mut result = (0, 0);

    for l in input.lines().map(|l| l.trim()) {
        let mut freq: HashMap<char, usize> = HashMap::new();
        for c in l.chars() {
            freq.entry(c).and_modify(|v| *v += 1).or_insert(1);
        }

        if freq.values().any(|v| *v == 2) {
            result.0 += 1;
        }
        if freq.values().any(|v| *v == 3) {
            result.1 += 1;
        }
    }

    result.0 * result.1
}

fn chars_diff(a: &str, b: &str) -> usize {
    let a = a.chars().collect::<Vec<char>>();
    let b = b.chars().collect::<Vec<char>>();
    let mut result = 0;

    for i in 0..a.len() {
        if a[i] != b[i] {
            result += 1;
        }
    }

    result
}

fn part2(input: &str) -> String {
    let input = input.lines().map(|l| l.trim()).collect::<Vec<&str>>();

    for i in 0..input.len() {
        for j in i + 1..input.len() {
            if chars_diff(input[i], input[j]) == 1 {
                let a = input[i].chars().collect::<Vec<char>>();
                let b = input[j].chars().collect::<Vec<char>>();
                let mut result = String::new();

                for k in 0..a.len() {
                    if a[k] == b[k] {
                        result.push(a[k]);
                    }
                }

                return result;
            }
        }
    }

    String::default()
}
