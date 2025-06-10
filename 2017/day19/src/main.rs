#[derive(Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn next(&self, grid: &[Vec<char>], current: &mut (usize, usize)) -> Option<char> {
        *current = match self {
            Self::Up => (current.0 - 1, current.1),
            Self::Down => (current.0 + 1, current.1),
            Self::Left => (current.0, current.1 - 1),
            Self::Right => (current.0, current.1 + 1),
        };

        grid.get(current.0)
            .unwrap_or(&vec![])
            .get(current.1)
            .copied()
    }
}

fn main() {
    let input = include_str!("../input");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> String {
    let grid = input
        .lines()
        .map(|l| l[..l.len() - 1].chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let mut current = (0_usize, grid[0].iter().position(|&c| c == '|').unwrap());

    let mut path = String::new();
    let mut direction = Direction::Down;

    loop {
        match (direction, direction.next(&grid, &mut current)) {
            (_, None | Some(' ')) => break,
            (Direction::Up | Direction::Down, Some('+')) => {
                let temp = grid[current.0].get(current.1 - 1);
                if temp.is_some() && temp != Some(&' ') {
                    direction = Direction::Left;
                } else {
                    direction = Direction::Right;
                }
            }
            (Direction::Left | Direction::Right, Some('+')) => {
                let temp = grid.get(current.0 - 1).map(|v| v.get(current.1).unwrap());
                if temp.is_some() && temp != Some(&' ') {
                    direction = Direction::Up;
                } else {
                    direction = Direction::Down;
                }
            }
            (_, Some('A'..='Z')) => path.push(grid[current.0][current.1]),
            _ => {}
        }
    }

    path
}

fn part2(input: &str) -> usize {
    let grid = input
        .lines()
        .map(|l| l[..l.len() - 1].chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let mut current = (0_usize, grid[0].iter().position(|&c| c == '|').unwrap());

    let mut steps = 1;
    let mut direction = Direction::Down;

    loop {
        match (direction, direction.next(&grid, &mut current)) {
            (_, None | Some(' ')) => break,
            (Direction::Up | Direction::Down, Some('+')) => {
                let temp = grid[current.0].get(current.1 - 1);
                if temp.is_some() && temp != Some(&' ') {
                    direction = Direction::Left;
                } else {
                    direction = Direction::Right;
                }
                steps += 1;
            }
            (Direction::Left | Direction::Right, Some('+')) => {
                let temp = grid.get(current.0 - 1).map(|v| v.get(current.1).unwrap());
                if temp.is_some() && temp != Some(&' ') {
                    direction = Direction::Up;
                } else {
                    direction = Direction::Down;
                }
                steps += 1;
            }
            _ => steps += 1,
        }
    }

    steps
}
