use std::collections::LinkedList;

#[derive(Clone, Copy)]
struct Marble {
    value: usize,
    prev: usize,
    next: usize,
}

fn main() {
    let input = include_str!("../input");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let input = input.split_whitespace().collect::<Vec<&str>>();
    let players = input[0].parse::<usize>().unwrap();
    let last = input[6].parse::<usize>().unwrap();

    let mut marbles: Vec<Marble> = Vec::from_iter([Marble {
        value: 0,
        prev: 0,
        next: 0,
    }]);
    let mut points: Vec<usize> = vec![0; players];

    let mut marble: usize = 1;
    let mut player: usize = 0;
    let mut current: usize = 0;

    while marble <= last {
        if marble % 23 == 0 {
            for _ in 0..7 {
                current = marbles[current].prev;
            }

            let m = marbles[current];
            marbles[m.prev].next = m.next;
            marbles[m.next].prev = m.prev;
            points[player] += marble + m.value;
            current = m.next;
        } else {
            current = marbles[current].next;
            let next = marbles[current].next;
            let prev = current;

            marbles.push(Marble {
                value: marble,
                prev,
                next,
            });
            marbles[prev].next = marbles.len() - 1;
            marbles[next].prev = marbles.len() - 1;
            current = marbles.len() - 1;
        }

        marble += 1;
        player = (player + 1) % players;
    }

    *points.iter().max().unwrap()
}

fn part2(input: &str) -> usize {
    let input = input.split_whitespace().collect::<Vec<&str>>();
    let players = input[0].parse::<usize>().unwrap();
    let last = input[6].parse::<usize>().unwrap() * 100;

    let mut marbles: Vec<Marble> = Vec::from_iter([Marble {
        value: 0,
        prev: 0,
        next: 0,
    }]);
    let mut points: Vec<usize> = vec![0; players];

    let mut marble: usize = 1;
    let mut player: usize = 0;
    let mut current: usize = 0;

    while marble <= last {
        if marble % 23 == 0 {
            for _ in 0..7 {
                current = marbles[current].prev;
            }

            let m = marbles[current];
            marbles[m.prev].next = m.next;
            marbles[m.next].prev = m.prev;
            points[player] += marble + m.value;
            current = m.next;
        } else {
            current = marbles[current].next;
            let next = marbles[current].next;
            let prev = current;

            marbles.push(Marble {
                value: marble,
                prev,
                next,
            });
            marbles[prev].next = marbles.len() - 1;
            marbles[next].prev = marbles.len() - 1;
            current = marbles.len() - 1;
        }

        marble += 1;
        player = (player + 1) % players;
    }

    *points.iter().max().unwrap()
}
