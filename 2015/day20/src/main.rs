use divisors::get_divisors;

fn main() {
    let input = include_str!("../input");

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let goal = input.trim().parse::<usize>().unwrap() / 10;
    let mut houses = 1;

    loop {
        let mut sum = 0;
        for div in get_divisors(houses) {
            sum += div;
        }
        sum += houses;

        if sum >= goal {
            return houses;
        }

        houses += 1;
    }
}

fn part2(input: &str) -> usize {
    let goal = input.trim().parse::<usize>().unwrap();
    let mut houses = 1;

    loop {
        let mut sum = 0;
        for div in get_divisors(houses) {
            if div * 50 >= houses {
                sum += div * 11;
            }
        }
        sum += houses * 11;

        if sum >= goal {
            return houses;
        }

        houses += 1;
    }
}
