use itertools::Itertools;

struct Node {
    location: (usize, usize),
    used: usize,
    available: usize,
}

fn main() {
    let input = include_str!("../input");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let mut nodes: Vec<Node> = vec![];

    for n in input
        .lines()
        .skip(2)
        .map(|l| l.split_whitespace().collect::<Vec<&str>>())
    {
        nodes.push(Node {
            location: (
                n[0][n[0].find("-x").unwrap() + 2..n[0].find("-y").unwrap()]
                    .parse::<usize>()
                    .unwrap(),
                n[0][n[0].find("-y").unwrap() + 2..]
                    .parse::<usize>()
                    .unwrap(),
            ),
            used: n[2][..n[2].len() - 1].parse::<usize>().unwrap(),
            available: n[3][..n[3].len() - 1].parse::<usize>().unwrap(),
        });
    }

    let mut result = 0;
    for (a, b) in nodes.iter().permutations(2).map(|p| (p[0], p[1])) {
        if a.used != 0 && a.location != b.location && a.used <= b.available {
            result += 1;
        }
    }

    result
}

fn part2(input: &str) -> String {
    let max_x = input
        .lines()
        .last()
        .unwrap()
        .chars()
        .skip_while(|&c| c != 'x')
        .skip(1)
        .take_while(|&c| c != '-')
        .collect::<String>()
        .parse::<usize>()
        .unwrap();
    let max_y = input
        .lines()
        .last()
        .unwrap()
        .chars()
        .skip_while(|&c| c != 'y')
        .skip(1)
        .take_while(|&c| c != ' ')
        .collect::<String>()
        .parse::<usize>()
        .unwrap();

    let mut grid: Vec<Vec<char>> = vec![vec!['x'; max_y + 1]; max_x + 1];

    for (pos, used) in input.lines().skip(2).map(|l| {
        let split = l.split_whitespace().collect::<Vec<&str>>();

        (
            (
                split[0]
                    .chars()
                    .skip_while(|&c| c != 'x')
                    .skip(1)
                    .take_while(|&c| c != '-')
                    .collect::<String>()
                    .parse::<usize>()
                    .unwrap(),
                split[0]
                    .chars()
                    .skip_while(|&c| c != 'y')
                    .skip(1)
                    .collect::<String>()
                    .parse::<usize>()
                    .unwrap(),
            ),
            split[2][..split[2].len() - 1].parse::<usize>().unwrap(),
        )
    }) {
        if pos.0 == max_x && pos.1 == 0 {
            grid[pos.0][pos.1] = 'G';
        } else if used >= 100 {
            grid[pos.0][pos.1] = '#';
        } else if used == 0 {
            grid[pos.0][pos.1] = '_';
        } else {
            grid[pos.0][pos.1] = '.';
        }
    }

    for row in grid {
        for c in row {
            print!("{c}");
        }
        println!();
    }

    String::from("Intended to solve by hand (get G to upper left corner) ^^^")
}
