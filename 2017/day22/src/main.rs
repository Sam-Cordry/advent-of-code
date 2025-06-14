use std::collections::VecDeque;

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn turn(&mut self, right: bool) {
        *self = match (*self, right) {
            (Self::Up, true) | (Self::Down, false) => Self::Right,
            (Self::Down, true) | (Self::Up, false) => Self::Left,
            (Self::Left, true) | (Self::Right, false) => Self::Up,
            (Self::Right, true) | (Self::Left, false) => Self::Down,
        };
    }

    pub fn reverse(&mut self) {
        *self = match *self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }
}

#[derive(Clone, PartialEq)]
enum NodeState {
    Clean,
    Weakened,
    Infected,
    Flagged,
}

impl NodeState {
    fn next(&mut self) {
        *self = match *self {
            Self::Clean => Self::Weakened,
            Self::Weakened => Self::Infected,
            Self::Infected => Self::Flagged,
            Self::Flagged => Self::Clean,
        }
    }
}

struct State1 {
    position: (usize, usize),
    facing: Direction,
    grid: VecDeque<VecDeque<bool>>,
}

impl State1 {
    fn new(start: &[Vec<char>]) -> Self {
        Self {
            position: (start.len() / 2, start[0].len() / 2),
            facing: Direction::Up,
            grid: start
                .iter()
                .map(|r| r.iter().map(|c| *c == '#').collect::<VecDeque<bool>>())
                .collect(),
        }
    }

    fn burst(&mut self, count: &mut usize) {
        self.facing
            .turn(self.grid[self.position.0][self.position.1]);
        self.grid[self.position.0][self.position.1] = !self.grid[self.position.0][self.position.1];

        if self.grid[self.position.0][self.position.1] {
            *count += 1;
        }

        match (self.facing, &mut self.position) {
            (Direction::Up, (0, _)) => {
                self.grid
                    .push_front(VecDeque::from_iter(vec![false; self.grid[0].len()]));
            }
            (Direction::Down, (x, _)) if *x == self.grid.len() - 1 => {
                self.grid
                    .push_back(VecDeque::from_iter(vec![false; self.grid[0].len()]));
                self.position.0 += 1;
            }
            (Direction::Left, (_, 0)) => {
                for i in 0..self.grid.len() {
                    self.grid[i].push_front(false);
                }
            }
            (Direction::Right, (_, y)) if *y == self.grid[0].len() - 1 => {
                for i in 0..self.grid.len() {
                    self.grid[i].push_back(false);
                }
                self.position.1 += 1;
            }
            (Direction::Up, (x, _)) => {
                *x -= 1;
            }
            (Direction::Down, (x, _)) => {
                *x += 1;
            }
            (Direction::Left, (_, y)) => {
                *y -= 1;
            }
            (Direction::Right, (_, y)) => {
                *y += 1;
            }
        }
    }
}

struct State2 {
    position: (usize, usize),
    facing: Direction,
    grid: VecDeque<VecDeque<NodeState>>,
}

impl State2 {
    fn new(grid: &[Vec<char>]) -> Self {
        Self {
            position: (grid.len() / 2, grid[0].len() / 2),
            facing: Direction::Up,
            grid: grid
                .iter()
                .map(|r| {
                    r.iter()
                        .map(|c| {
                            if *c == '#' {
                                NodeState::Infected
                            } else {
                                NodeState::Clean
                            }
                        })
                        .collect()
                })
                .collect(),
        }
    }

    fn burst(&mut self, count: &mut usize) {
        match self.grid[self.position.0][self.position.1] {
            NodeState::Clean => self.facing.turn(false),
            NodeState::Weakened => {}
            NodeState::Infected => self.facing.turn(true),
            NodeState::Flagged => self.facing.reverse(),
        }

        self.grid[self.position.0][self.position.1].next();

        if self.grid[self.position.0][self.position.1] == NodeState::Infected {
            *count += 1;
        }

        match (self.facing, &mut self.position) {
            (Direction::Up, (0, _)) => {
                self.grid.push_front(VecDeque::from_iter(vec![
                    NodeState::Clean;
                    self.grid[0].len()
                ]));
            }
            (Direction::Down, (x, _)) if *x == self.grid.len() - 1 => {
                self.grid.push_back(VecDeque::from_iter(vec![
                    NodeState::Clean;
                    self.grid[0].len()
                ]));
                self.position.0 += 1;
            }
            (Direction::Left, (_, 0)) => {
                for i in 0..self.grid.len() {
                    self.grid[i].push_front(NodeState::Clean);
                }
            }
            (Direction::Right, (_, y)) if *y == self.grid[0].len() - 1 => {
                for i in 0..self.grid.len() {
                    self.grid[i].push_back(NodeState::Clean);
                }
                self.position.1 += 1;
            }
            (Direction::Up, (x, _)) => {
                *x -= 1;
            }
            (Direction::Down, (x, _)) => {
                *x += 1;
            }
            (Direction::Left, (_, y)) => {
                *y -= 1;
            }
            (Direction::Right, (_, y)) => {
                *y += 1;
            }
        }
    }
}

fn main() {
    let input = include_str!("../input");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let mut state = State1::new(
        &input
            .lines()
            .map(|l| l.chars().collect())
            .collect::<Vec<Vec<char>>>(),
    );
    let mut result = 0;

    for _ in 0..10000 {
        state.burst(&mut result);
    }

    result
}

fn part2(input: &str) -> usize {
    let mut state = State2::new(
        &input
            .lines()
            .map(|l| l.chars().collect())
            .collect::<Vec<Vec<char>>>(),
    );
    let mut result = 0;

    for _ in 0..10000000 {
        state.burst(&mut result);
    }

    result
}
