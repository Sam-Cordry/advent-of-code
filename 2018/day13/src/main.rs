use std::collections::{HashMap, HashSet};

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Hash, PartialEq, Eq, Clone)]
struct Cart {
    position: (usize, usize),
    direction: Direction,
    intersections: usize,
}

impl Cart {
    fn advance(&mut self) {
        self.position = match self.direction {
            Direction::Up => (self.position.0, self.position.1 - 1),
            Direction::Down => (self.position.0, self.position.1 + 1),
            Direction::Left => (self.position.0 - 1, self.position.1),
            Direction::Right => (self.position.0 + 1, self.position.1),
        }
    }

    fn turn(&mut self, right: bool) {
        self.direction = match (self.direction, right) {
            (Direction::Up, true) | (Direction::Down, false) => Direction::Right,
            (Direction::Up, false) | (Direction::Down, true) => Direction::Left,
            (Direction::Left, true) | (Direction::Right, false) => Direction::Up,
            (Direction::Left, false) | (Direction::Right, true) => Direction::Down,
        }
    }
}

fn main() {
    let input = include_str!("../input");
    println!("Part 1: {:?}", part1(input));
    println!("Part 2: {:?}", part2(input));
}

fn parse(track: &mut [Vec<char>], carts: &mut Vec<Cart>) {
    for i in 0..track.len() {
        for j in 0..track[0].len() {
            match track[i][j] {
                '^' => {
                    carts.push(Cart {
                        position: (j, i),
                        direction: Direction::Up,
                        intersections: 0,
                    });
                    track[i][j] = '|';
                }
                'v' => {
                    carts.push(Cart {
                        position: (j, i),
                        direction: Direction::Down,
                        intersections: 0,
                    });
                    track[i][j] = '|';
                }
                '<' => {
                    carts.push(Cart {
                        position: (j, i),
                        direction: Direction::Left,
                        intersections: 0,
                    });
                    track[i][j] = '-';
                }
                '>' => {
                    carts.push(Cart {
                        position: (j, i),
                        direction: Direction::Right,
                        intersections: 0,
                    });
                    track[i][j] = '-';
                }
                _ => {}
            }
        }
    }
}

fn part1(input: &str) -> (usize, usize) {
    let mut track = input
        .lines()
        .map(|l| l.chars().collect())
        .collect::<Vec<Vec<char>>>();
    let mut carts: Vec<Cart> = vec![];

    parse(&mut track, &mut carts);

    while HashSet::<(usize, usize)>::from_iter(carts.iter().map(|c| c.position)).len()
        == carts.len()
    {
        for i in 0..carts.len() {
            carts[i].advance();
            match track[carts[i].position.1][carts[i].position.0] {
                '/' => {
                    if carts[i].direction == Direction::Up || carts[i].direction == Direction::Down
                    {
                        carts[i].turn(true);
                    } else {
                        carts[i].turn(false);
                    }
                }
                '\\' => {
                    if carts[i].direction == Direction::Up || carts[i].direction == Direction::Down
                    {
                        carts[i].turn(false);
                    } else {
                        carts[i].turn(true);
                    }
                }
                '+' => {
                    match carts[i].intersections % 3 {
                        0 => carts[i].turn(false),
                        2 => carts[i].turn(true),
                        _ => {}
                    }
                    carts[i].intersections += 1;
                }
                _ => {}
            }

            if carts
                .iter()
                .filter(|c| c.position == carts[i].position)
                .count()
                > 1
            {
                break;
            }
        }
    }

    let mut freq: HashMap<(usize, usize), usize> = HashMap::new();
    for c in carts {
        freq.entry(c.position).and_modify(|n| *n += 1).or_insert(1);
    }

    *freq.iter().find(|e| *e.1 > 1).unwrap().0
}

fn part2(input: &str) -> (usize, usize) {
    let mut track = input
        .lines()
        .map(|l| l.chars().collect())
        .collect::<Vec<Vec<char>>>();
    let mut carts: Vec<Cart> = vec![];

    parse(&mut track, &mut carts);
    carts.sort_by(|a, b| {
        if a.position.1 != b.position.1 {
            a.position.1.cmp(&b.position.1)
        } else {
            a.position.0.cmp(&b.position.0)
        }
    });

    loop {
        let mut i = 0;
        loop {
            carts[i].advance();
            match track[carts[i].position.1][carts[i].position.0] {
                '/' => {
                    if carts[i].direction == Direction::Up || carts[i].direction == Direction::Down
                    {
                        carts[i].turn(true);
                    } else {
                        carts[i].turn(false);
                    }
                }
                '\\' => {
                    if carts[i].direction == Direction::Up || carts[i].direction == Direction::Down
                    {
                        carts[i].turn(false);
                    } else {
                        carts[i].turn(true);
                    }
                }
                '+' => {
                    match carts[i].intersections % 3 {
                        0 => carts[i].turn(false),
                        2 => carts[i].turn(true),
                        _ => {}
                    }
                    carts[i].intersections += 1;
                }
                _ => {}
            }

            if carts
                .iter()
                .filter(|c| c.position == carts[i].position)
                .count()
                > 1
            {
                let p = carts[i].position;
                for _ in 0..2 {
                    let r = carts.iter().position(|c| c.position == p).unwrap();
                    carts.remove(r);
                    if r <= i {
                        i -= 1;
                    }
                }
            }

            i += 1;

            if i == carts.len() {
                break;
            } else if i > carts.len() {
                i = 0;
            }
        }

        if carts.len() == 1 {
            break;
        }

        carts.sort_by(|a, b| {
            if a.position.1 != b.position.1 {
                a.position.1.cmp(&b.position.1)
            } else {
                a.position.0.cmp(&b.position.0)
            }
        });
    }

    carts[0].position
}
