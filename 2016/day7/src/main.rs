use std::collections::HashSet;

fn main() {
    let input = include_str!("../input");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn contains_reversed_pair(s: &str) -> bool {
    if s.len() < 4 {
        return false;
    }

    let chars = s.chars().collect::<Vec<char>>();
    for i in 0..chars.len() - 3 {
        if chars[i] == chars[i + 3] && chars[i + 1] == chars[i + 2] && chars[i] != chars[i + 1] {
            return true;
        }
    }

    false
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|l| l.trim().split(['[', ']']).collect::<Vec<&str>>())
        .filter(|l| {
            let mut result = false;
            for (i, s) in l.iter().enumerate() {
                if i % 2 == 1 && contains_reversed_pair(s) {
                    return false;
                } else if contains_reversed_pair(s) {
                    result = true;
                }
            }
            result
        })
        .count()
}

fn is_ssl(supers: &[&str], hypers: &[&str]) -> bool {
    for s in supers {
        for t in s
            .chars()
            .collect::<Vec<char>>()
            .windows(3)
            .filter(|w| w[0] == w[2] && w[0] != w[1])
        {
            for h in hypers {
                if h.chars()
                    .collect::<Vec<char>>()
                    .windows(3)
                    .filter(|w| w[0] == w[2] && w[0] != w[1] && t[0] == w[1] && t[1] == w[0])
                    .count()
                    != 0
                {
                    return true;
                }
            }
        }
    }

    false
}

fn part2(input: &str) -> usize {
    let mut count = 0;

    for (supers, hypers) in input
        .lines()
        .map(|l| l.trim().split(['[', ']']).collect::<Vec<&str>>())
        .filter(|l| {
            let mut count = 0;
            for s in l {
                if HashSet::<char>::from_iter(s.chars()).len() < s.len() {
                    count += 1;
                }
            }
            count > 1
        })
        .map(|l| {
            let mut even: Vec<&str> = vec![];
            let mut odd: Vec<&str> = vec![];

            for (i, s) in l.iter().enumerate() {
                if i % 2 == 0 {
                    even.push(s);
                } else {
                    odd.push(s);
                }
            }

            (even, odd)
        })
    {
        if is_ssl(&supers, &hypers) {
            count += 1;
        }
    }

    count
}
