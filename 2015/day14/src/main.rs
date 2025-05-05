enum State {
    Flying(usize),
    Resting(usize),
}

struct Reindeer {
    distance: usize,
    fly_time: usize,
    rest_time: usize,
    speed: usize,
    state: State,
    points: usize,
}

impl Reindeer {
    fn advance(&mut self) {
        match self.state {
            State::Flying(time) => {
                self.distance += self.speed;
                if time == 1 {
                    self.state = State::Resting(self.rest_time);
                } else {
                    self.state = State::Flying(time - 1);
                }
            }
            State::Resting(time) => {
                if time == 1 {
                    self.state = State::Flying(self.fly_time);
                } else {
                    self.state = State::Resting(time - 1);
                }
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
    let mut reindeer: Vec<Reindeer> = vec![];

    for line in input
        .lines()
        .map(|l| l.split_whitespace().collect::<Vec<&str>>())
    {
        reindeer.push(Reindeer {
            distance: 0,
            fly_time: line[6].parse::<usize>().unwrap(),
            rest_time: line[13].parse::<usize>().unwrap(),
            speed: line[3].parse::<usize>().unwrap(),
            state: State::Flying(line[6].parse::<usize>().unwrap()),
            points: 0,
        });
    }

    for _ in 0..2503 {
        for r in reindeer.iter_mut() {
            r.advance();
        }
    }

    reindeer.iter().map(|r| r.distance).max().unwrap()
}

fn part2(input: &str) -> usize {
    let mut reindeer: Vec<Reindeer> = vec![];

    for line in input
        .lines()
        .map(|l| l.split_whitespace().collect::<Vec<&str>>())
    {
        reindeer.push(Reindeer {
            distance: 0,
            fly_time: line[6].parse::<usize>().unwrap(),
            rest_time: line[13].parse::<usize>().unwrap(),
            speed: line[3].parse::<usize>().unwrap(),
            state: State::Flying(line[6].parse::<usize>().unwrap()),
            points: 0,
        });
    }

    for _ in 0..2503 {
        for r in reindeer.iter_mut() {
            r.advance();
        }

        let max = reindeer.iter().map(|r| r.distance).max().unwrap();
        for r in reindeer.iter_mut() {
            if r.distance == max {
                r.points += 1;
            }
        }
    }

    reindeer.iter().map(|r| r.points).max().unwrap()
}
