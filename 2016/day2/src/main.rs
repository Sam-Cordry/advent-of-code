use std::fmt::Display;

#[derive(Clone, Copy)]
enum Key1 {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl Display for Key1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::One => write!(f, "1"),
            Self::Two => write!(f, "2"),
            Self::Three => write!(f, "3"),
            Self::Four => write!(f, "4"),
            Self::Five => write!(f, "5"),
            Self::Six => write!(f, "6"),
            Self::Seven => write!(f, "7"),
            Self::Eight => write!(f, "8"),
            Self::Nine => write!(f, "9"),
        }
    }
}

impl Key1 {
    fn next(self, direction: &char) -> Self {
        match (self, direction) {
            (Self::One, 'U' | 'L') | (Self::Four, 'U') | (Self::Two, 'L') => Self::One,
            (Self::Two | Self::Five, 'U') | (Self::One, 'R') | (Self::Three, 'L') => Self::Two,
            (Self::Three, 'U' | 'R') | (Self::Two, 'R') | (Self::Six, 'U') => Self::Three,
            (Self::Four | Self::Five, 'L') | (Self::One, 'D') | (Self::Seven, 'U') => Self::Four,
            (Self::Two, 'D') | (Self::Four, 'R') | (Self::Six, 'L') | (Self::Eight, 'U') => {
                Self::Five
            }
            (Self::Six | Self::Five, 'R') | (Self::Three, 'D') | (Self::Nine, 'U') => Self::Six,
            (Self::Seven, 'D' | 'L') | (Self::Four, 'D') | (Self::Eight, 'L') => Self::Seven,
            (Self::Eight | Self::Five, 'D') | (Self::Seven, 'R') | (Self::Nine, 'L') => Self::Eight,
            (Self::Nine, 'D' | 'R') | (Self::Six, 'D') | (Self::Eight, 'R') => Self::Nine,
            _ => panic!("invalid direction"),
        }
    }
}

enum Key2 {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    A,
    B,
    C,
    D,
}

impl Display for Key2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::One => write!(f, "1"),
            Self::Two => write!(f, "2"),
            Self::Three => write!(f, "3"),
            Self::Four => write!(f, "4"),
            Self::Five => write!(f, "5"),
            Self::Six => write!(f, "6"),
            Self::Seven => write!(f, "7"),
            Self::Eight => write!(f, "8"),
            Self::Nine => write!(f, "9"),
            Self::A => write!(f, "A"),
            Self::B => write!(f, "B"),
            Self::C => write!(f, "C"),
            Self::D => write!(f, "D"),
        }
    }
}

impl Key2 {
    fn next(self, direction: &char) -> Self {
        match (self, direction) {
            (Self::One, 'U' | 'L' | 'R') | (Self::Three, 'U') => Self::One,
            (Self::Two, 'U' | 'L') | (Self::Three, 'L') | (Self::Six, 'U') => Self::Two,
            (Self::One, 'D') | (Self::Two, 'R') | (Self::Four, 'L') | (Self::Seven, 'U') => {
                Self::Three
            }
            (Self::Four, 'U' | 'R') | (Self::Three, 'R') | (Self::Eight, 'U') => Self::Four,
            (Self::Five, 'U' | 'L' | 'D') | (Self::Six, 'L') => Self::Five,
            (Self::Two, 'D') | (Self::Five, 'R') | (Self::Seven, 'L') | (Self::A, 'U') => Self::Six,
            (Self::Three, 'D') | (Self::Six, 'R') | (Self::Eight, 'L') | (Self::B, 'U') => {
                Self::Seven
            }
            (Self::Four, 'D') | (Self::Seven, 'R') | (Self::Nine, 'L') | (Self::C, 'U') => {
                Self::Eight
            }
            (Self::Nine, 'U' | 'R' | 'D') | (Self::Eight, 'R') => Self::Nine,
            (Self::A, 'D' | 'L') | (Self::Six, 'D') | (Self::B, 'L') => Self::A,
            (Self::Seven, 'D') | (Self::A, 'R') | (Self::C, 'L') | (Self::D, 'U') => Self::B,
            (Self::C, 'D' | 'R') | (Self::Eight, 'D') | (Self::B, 'R') => Self::C,
            (Self::D, 'D' | 'L' | 'R') | (Self::B, 'D') => Self::D,
            _ => panic!("invalid direction"),
        }
    }
}

fn main() {
    let input = include_str!("../input");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> String {
    let mut result = String::new();
    let mut current = Key1::Five;

    for line in input.lines().map(|l| l.trim()) {
        for c in line.chars() {
            current = current.next(&c);
        }
        result.push_str(&current.to_string());
    }

    result
}

fn part2(input: &str) -> String {
    let mut result = String::new();
    let mut current = Key2::Five;

    for line in input.lines().map(|l| l.trim()) {
        for c in line.chars() {
            current = current.next(&c);
        }
        result.push_str(&current.to_string());
    }

    result
}
