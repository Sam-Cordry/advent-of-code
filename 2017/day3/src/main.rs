fn main() {
    let input = include_str!("../input");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let goal = input.trim().parse::<usize>().unwrap();

    for (i, x) in (1..).filter(|n| n % 2 == 1).enumerate() {
        if goal <= x * x {
            return i
                + (0..4)
                    .map(|v| goal.abs_diff((x * x - x / 2) - (x - 1) * v))
                    .min()
                    .unwrap();
        }
    }

    unreachable!()
}

fn calculate(grid: &[Vec<usize>], x: usize, y: usize) -> usize {
    grid[x - 1][y - 1]
        + grid[x - 1][y]
        + grid[x - 1][y + 1]
        + grid[x][y - 1]
        + grid[x][y + 1]
        + grid[x + 1][y - 1]
        + grid[x + 1][y]
        + grid[x + 1][y + 1]
}

fn part2(input: &str) -> usize {
    let goal = input.trim().parse::<usize>().unwrap();

    let mut grid: Vec<Vec<usize>> = vec![vec![0; 15]; 15];

    let mut x = 7;
    let mut y = 7;
    grid[x][y] = 1;

    x += 1;
    grid[x][y] = 1;

    for side in (3..).filter(|n| n % 2 == 1) {
        for _ in 0..side - 2 {
            y += 1;
            grid[x][y] = calculate(&grid, x, y);
        }
        for _ in 0..side - 1 {
            x -= 1;
            grid[x][y] = calculate(&grid, x, y);
        }
        for _ in 0..side - 1 {
            y -= 1;
            grid[x][y] = calculate(&grid, x, y);
        }
        for _ in 0..side {
            x += 1;
            grid[x][y] = calculate(&grid, x, y);
        }

        if grid.iter().any(|r| r.iter().any(|&n| n > goal)) {
            break;
        }
    }

    *grid
        .iter()
        .map(|r| r.iter().filter(|&&n| n > goal).min().unwrap_or(&usize::MAX))
        .min()
        .unwrap()
}
