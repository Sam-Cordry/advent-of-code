fn main() {
    let input = include_str!("../input");

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn next(current: &str) -> String {
    let mut chars: Vec<char> = current.chars().collect();
    let mut i = chars.len() - 1;
    loop {
        if chars[i] == 'z' {
            chars[i] = 'a';
            i -= 1;
        } else if chars[i] == 'h' {
            chars[i] = 'j';
        } else if chars[i] == 'n' {
            chars[i] = 'p';
        } else if chars[i] == 'k' {
            chars[i] = 'm';
        } else {
            chars[i] = (chars[i] as u8 + 1) as char;
            break;
        }
    }
    chars.into_iter().collect()
}

fn has_two_pair(current: &str) -> bool {
    let mut chars: Vec<char> = current.chars().collect();
    let mut index: Option<usize> = None;

    for i in 0..chars.len() - 1 {
        if chars[i] == chars[i + 1] {
            index = Some(i);
            break;
        }
    }

    if index.is_none() {
        return false;
    }

    chars.remove(index.unwrap());
    chars.remove(index.unwrap());
    chars.insert(index.unwrap(), ' ');

    for i in 0..chars.len() - 1 {
        if chars[i] == chars[i + 1] {
            return true;
        }
    }
    false
}

fn is_valid(current: &str) -> bool {
    current
        .chars()
        .collect::<Vec<char>>()
        .windows(3)
        .any(|t| t[1] == (t[0] as u8 + 1) as char && t[2] == (t[0] as u8 + 2) as char)
        && !current.chars().any(|c| c == 'i' || c == 'o' || c == 'l')
        && has_two_pair(current)
}

fn part1(input: &str) -> String {
    let mut current = input.trim().to_string();

    loop {
        current = next(&current);
        if is_valid(&current) {
            break;
        }
    }

    current
}

fn part2(input: &str) -> String {
    let mut current = part1(input);

    loop {
        current = next(&current);
        if is_valid(&current) {
            break;
        }
    }

    current
}
