fn main() {
    let input = include_str!("../input");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> String {
    let mut recipes: Vec<u8> = vec![3, 7];
    let mut elves = (0, 1);

    while recipes.len() < input.trim().parse::<usize>().unwrap() + 10 {
        recipes.append(
            &mut (recipes[elves.0] + recipes[elves.1])
                .to_string()
                .chars()
                .map(|c| c.to_string().parse::<u8>().unwrap())
                .collect(),
        );
        elves.0 = (elves.0 + 1 + recipes[elves.0] as usize) % recipes.len();
        elves.1 = (elves.1 + 1 + recipes[elves.1] as usize) % recipes.len();
    }

    recipes
        .iter()
        .skip(input.trim().parse().unwrap())
        .take(10)
        .map(|r| r.to_string().chars().next().unwrap())
        .collect::<String>()
}

fn part2(input: &str) -> usize {
    let mut recipes: Vec<u8> = vec![3, 7];
    let mut elves = (0, 1);
    let needle: Vec<u8> = input
        .trim()
        .chars()
        .map(|c| c.to_string().parse::<u8>().unwrap())
        .collect();

    let mut p = 0;

    loop {
        recipes.append(
            &mut (recipes[elves.0] + recipes[elves.1])
                .to_string()
                .chars()
                .map(|c| c.to_string().parse::<u8>().unwrap())
                .collect(),
        );
        elves.0 = (elves.0 + 1 + recipes[elves.0] as usize) % recipes.len();
        elves.1 = (elves.1 + 1 + recipes[elves.1] as usize) % recipes.len();

        while p + needle.len() < recipes.len() {
            if recipes[p..p + needle.len()] == needle {
                return p;
            }
            p += 1;
        }
    }
}
