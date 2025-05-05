#[derive(Default)]
struct Sue {
    children: Option<usize>,
    cats: Option<usize>,
    samoyeds: Option<usize>,
    pomeranians: Option<usize>,
    akitas: Option<usize>,
    vizslas: Option<usize>,
    goldfish: Option<usize>,
    trees: Option<usize>,
    cars: Option<usize>,
    perfumes: Option<usize>,
}

fn main() {
    let input = include_str!("../input");

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn parse_as_usize(str: &str) -> usize {
    if let Some(s) = str.strip_suffix(',') {
        s.parse().unwrap()
    } else {
        str.parse().unwrap()
    }
}

fn part1(input: &str) -> usize {
    let mut sues: Vec<Sue> = vec![];

    for line in input
        .lines()
        .map(|l| l.split_whitespace().collect::<Vec<&str>>())
    {
        let mut sue = Sue::default();

        for (i, s) in line.iter().enumerate() {
            match *s {
                "children:" => sue.children = Some(parse_as_usize(line[i + 1])),
                "cats:" => sue.cats = Some(parse_as_usize(line[i + 1])),
                "samoyeds:" => sue.samoyeds = Some(parse_as_usize(line[i + 1])),
                "pomeranians:" => sue.pomeranians = Some(parse_as_usize(line[i + 1])),
                "akitas:" => sue.akitas = Some(parse_as_usize(line[i + 1])),
                "vizslas:" => sue.vizslas = Some(parse_as_usize(line[i + 1])),
                "goldfish:" => sue.goldfish = Some(parse_as_usize(line[i + 1])),
                "trees:" => sue.trees = Some(parse_as_usize(line[i + 1])),
                "cars:" => sue.cars = Some(parse_as_usize(line[i + 1])),
                "perfumes:" => sue.perfumes = Some(parse_as_usize(line[i + 1])),
                _ => {}
            }
        }

        sues.push(sue);
    }

    sues.iter()
        .position(|s| {
            s.children.is_none_or(|x| x == 3)
                && s.cats.is_none_or(|x| x == 7)
                && s.samoyeds.is_none_or(|x| x == 2)
                && s.pomeranians.is_none_or(|x| x == 3)
                && s.akitas.is_none_or(|x| x == 0)
                && s.vizslas.is_none_or(|x| x == 0)
                && s.goldfish.is_none_or(|x| x == 5)
                && s.trees.is_none_or(|x| x == 3)
                && s.cars.is_none_or(|x| x == 2)
                && s.perfumes.is_none_or(|x| x == 1)
        })
        .unwrap()
        + 1
}

fn part2(input: &str) -> usize {
    let mut sues: Vec<Sue> = vec![];

    for line in input
        .lines()
        .map(|l| l.split_whitespace().collect::<Vec<&str>>())
    {
        let mut sue = Sue::default();

        for (i, s) in line.iter().enumerate() {
            match *s {
                "children:" => sue.children = Some(parse_as_usize(line[i + 1])),
                "cats:" => sue.cats = Some(parse_as_usize(line[i + 1])),
                "samoyeds:" => sue.samoyeds = Some(parse_as_usize(line[i + 1])),
                "pomeranians:" => sue.pomeranians = Some(parse_as_usize(line[i + 1])),
                "akitas:" => sue.akitas = Some(parse_as_usize(line[i + 1])),
                "vizslas:" => sue.vizslas = Some(parse_as_usize(line[i + 1])),
                "goldfish:" => sue.goldfish = Some(parse_as_usize(line[i + 1])),
                "trees:" => sue.trees = Some(parse_as_usize(line[i + 1])),
                "cars:" => sue.cars = Some(parse_as_usize(line[i + 1])),
                "perfumes:" => sue.perfumes = Some(parse_as_usize(line[i + 1])),
                _ => {}
            }
        }

        sues.push(sue);
    }

    sues.iter()
        .position(|s| {
            s.children.is_none_or(|x| x == 3)
                && s.cats.is_none_or(|x| x > 7)
                && s.samoyeds.is_none_or(|x| x == 2)
                && s.pomeranians.is_none_or(|x| x < 3)
                && s.akitas.is_none_or(|x| x == 0)
                && s.vizslas.is_none_or(|x| x == 0)
                && s.goldfish.is_none_or(|x| x < 5)
                && s.trees.is_none_or(|x| x > 3)
                && s.cars.is_none_or(|x| x == 2)
                && s.perfumes.is_none_or(|x| x == 1)
        })
        .unwrap()
        + 1
}
