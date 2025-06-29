struct Point {
    position: (i32, i32),
    velocity: (i32, i32),
}

impl Point {
    fn position_at(&self, t: i32) -> (i32, i32) {
        (
            self.position.0 + self.velocity.0 * t,
            self.position.1 + self.velocity.1 * t,
        )
    }
}

fn main() {
    let input = include_str!("../input");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> String {
    let points: Vec<Point> = input
        .lines()
        .map(|l| {
            let split = l.split(['<', '>', ',']).collect::<Vec<&str>>();

            Point {
                position: (
                    split[1].trim().parse().unwrap(),
                    split[2].trim().parse().unwrap(),
                ),
                velocity: (
                    split[4].trim().parse().unwrap(),
                    split[5].trim().parse().unwrap(),
                ),
            }
        })
        .collect();

    let mut min_time = (0usize, usize::MAX);
    for t in 0..100000 {
        let coords: Vec<i32> = points.iter().map(|p| p.position_at(t).1).collect();
        let diff = (coords.iter().max().unwrap() - coords.iter().min().unwrap()) as usize;

        if diff < min_time.1 {
            min_time = (t as usize, diff);
        }
    }

    let coords: Vec<(i32, i32)> = points
        .iter()
        .map(|p| p.position_at(min_time.0 as i32))
        .collect();
    let min = (
        coords.iter().map(|p| p.0).min().unwrap(),
        coords.iter().map(|p| p.1).min().unwrap(),
    );
    let max = (
        coords.iter().map(|p| p.0).max().unwrap(),
        coords.iter().map(|p| p.1).max().unwrap(),
    );

    let mut grid = vec![vec![' '; (max.0 - min.0) as usize + 1]; (max.1 - min.1) as usize + 1];

    for (x, y) in coords {
        grid[(y - min.1) as usize][(x - min.0) as usize] = '#';
    }

    for r in grid {
        for c in r {
            print!("{c}");
        }
        println!();
    }

    String::from("Consider the above for message")
}

fn part2(input: &str) -> usize {
    let points: Vec<Point> = input
        .lines()
        .map(|l| {
            let split = l.split(['<', '>', ',']).collect::<Vec<&str>>();

            Point {
                position: (
                    split[1].trim().parse().unwrap(),
                    split[2].trim().parse().unwrap(),
                ),
                velocity: (
                    split[4].trim().parse().unwrap(),
                    split[5].trim().parse().unwrap(),
                ),
            }
        })
        .collect();

    let mut min_time = (0usize, usize::MAX);
    for t in 0..100000 {
        let coords: Vec<i32> = points.iter().map(|p| p.position_at(t).1).collect();
        let diff = (coords.iter().max().unwrap() - coords.iter().min().unwrap()) as usize;

        if diff < min_time.1 {
            min_time = (t as usize, diff);
        }
    }

    min_time.0
}
