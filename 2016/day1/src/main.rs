use std::collections::HashSet;

enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn turn(self, direction: &char) -> Self {
        match (self, direction) {
            (Self::North, 'L') | (Self::South, 'R') => Self::West,
            (Self::North, 'R') | (Self::South, 'L') => Self::East,
            (Self::East, 'L') | (Self::West, 'R') => Self::North,
            (Self::East, 'R') | (Self::West, 'L') => Self::South,
            _ => panic!("invalid direction"),
        }
    }
}

fn main() {
    let input = include_str!("../input");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let instructions = input.trim().split(", ").collect::<Vec<&str>>();

    let mut direction = Direction::North;
    let mut x = 0;
    let mut y = 0;
    for i in instructions {
        direction = direction.turn(&i.chars().next().unwrap());

        let temp = i[1..].parse::<i32>().unwrap();
        match direction {
            Direction::North => y += temp,
            Direction::South => y -= temp,
            Direction::East => x += temp,
            Direction::West => x -= temp,
        }
    }

    (x.abs() + y.abs()) as usize
}

fn part2(input: &str) -> usize {
    let instructions = input.trim().split(", ").collect::<Vec<&str>>();

    let mut direction = Direction::North;
    let mut x = 0;
    let mut y = 0;
    let mut seen: HashSet<(i32, i32)> = HashSet::new();
    for i in instructions {
        direction = direction.turn(&i.chars().next().unwrap());

        for _ in 0..i[1..].parse::<usize>().unwrap() {
            match direction {
                Direction::North => y += 1,
                Direction::South => y -= 1,
                Direction::East => x += 1,
                Direction::West => x -= 1,
            }

            if seen.contains(&(x, y)) {
                return (x.abs() + y.abs()) as usize;
            }

            seen.insert((x, y));
        }
    }

    0
}
