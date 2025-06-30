fn main() {
    let input = include_str!("../input");
    println!("Part 1: {:?}", part1(input));
    println!("Part 2: {:?}", part2(input));
}

fn part1(input: &str) -> (usize, usize) {
    let mut grid = vec![vec![0i32; 300]; 300];

    for (i, row) in grid.iter_mut().enumerate() {
        for (j, v) in row.iter_mut().enumerate() {
            *v = ((((((j as i32 + 11) * (i as i32 + 1)) + input.trim().parse::<i32>().unwrap())
                * (j as i32 + 11))
                % 1000)
                / 100)
                - 5;
        }
    }

    let mut max = (i32::MIN, (0usize, 0usize));

    for i in 0..298 {
        for j in 0..298 {
            let power = grid
                .iter()
                .skip(i)
                .take(3)
                .map(|r| r.iter().skip(j).take(3).sum::<i32>())
                .sum::<i32>();

            if power > max.0 {
                max = (power, (j + 1, i + 1));
            }
        }
    }

    max.1
}

fn part2(input: &str) -> (usize, usize, usize) {
    let mut grid = vec![vec![0i32; 300]; 300];

    for (i, row) in grid.iter_mut().enumerate() {
        for (j, v) in row.iter_mut().enumerate() {
            *v = ((((((j as i32 + 11) * (i as i32 + 1)) + input.trim().parse::<i32>().unwrap())
                * (j as i32 + 11))
                % 1000)
                / 100)
                - 5;
        }
    }

    let mut max = (i32::MIN, (0usize, 0usize, 0usize));

    for size in 1..=300 {
        for i in 0..(301 - size) {
            for j in 0..(301 - size) {
                let power = grid
                    .iter()
                    .skip(i)
                    .take(size)
                    .map(|r| r.iter().skip(j).take(size).sum::<i32>())
                    .sum::<i32>();

                if power > max.0 {
                    max = (power, (j + 1, i + 1, size));
                }
            }
        }
    }

    max.1
}
