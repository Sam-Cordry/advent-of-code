struct Ingredient {
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

fn main() {
    let input = include_str!("../input");

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn find_distributions(
    sum_left: usize,
    elements_left: usize,
    current: &mut Vec<usize>,
    result: &mut Vec<Vec<usize>>,
    index: usize,
) {
    if elements_left == 0 {
        if sum_left == 0 {
            result.push(current.clone());
        }
        return;
    }

    for i in 0..=sum_left {
        current[index] = i;
        find_distributions(sum_left - i, elements_left - 1, current, result, index + 1);
    }
}

fn part1(input: &str) -> usize {
    let mut ingredients: Vec<Ingredient> = vec![];

    for line in input
        .lines()
        .map(|l| l.split_whitespace().collect::<Vec<&str>>())
    {
        ingredients.push(Ingredient {
            capacity: line[2][..line[2].len() - 1].parse().unwrap(),
            durability: line[4][..line[4].len() - 1].parse().unwrap(),
            flavor: line[6][..line[6].len() - 1].parse().unwrap(),
            texture: line[8][..line[8].len() - 1].parse().unwrap(),
            calories: line[10].parse().unwrap(),
        })
    }

    let mut result = vec![];
    find_distributions(
        100,
        ingredients.len(),
        &mut vec![0; ingredients.len()],
        &mut result,
        0,
    );

    let mut max: usize = 0;
    for dist in result {
        let mut capacity = 0;
        let mut durability = 0;
        let mut flavor = 0;
        let mut texture = 0;

        for i in 0..dist.len() {
            capacity += ingredients[i].capacity * dist[i] as i32;
            durability += ingredients[i].durability * dist[i] as i32;
            flavor += ingredients[i].flavor * dist[i] as i32;
            texture += ingredients[i].texture * dist[i] as i32;
        }

        if capacity > 0
            && durability > 0
            && flavor > 0
            && texture > 0
            && (capacity * durability * flavor * texture) as usize > max
        {
            max = (capacity * durability * flavor * texture) as usize;
        }
    }

    max
}

fn part2(input: &str) -> usize {
    let mut ingredients: Vec<Ingredient> = vec![];

    for line in input
        .lines()
        .map(|l| l.split_whitespace().collect::<Vec<&str>>())
    {
        ingredients.push(Ingredient {
            capacity: line[2][..line[2].len() - 1].parse().unwrap(),
            durability: line[4][..line[4].len() - 1].parse().unwrap(),
            flavor: line[6][..line[6].len() - 1].parse().unwrap(),
            texture: line[8][..line[8].len() - 1].parse().unwrap(),
            calories: line[10].parse().unwrap(),
        })
    }

    let mut result = vec![];
    find_distributions(
        100,
        ingredients.len(),
        &mut vec![0; ingredients.len()],
        &mut result,
        0,
    );

    let mut max: usize = 0;
    for dist in result {
        let mut capacity = 0;
        let mut durability = 0;
        let mut flavor = 0;
        let mut texture = 0;
        let mut calories = 0;

        for i in 0..dist.len() {
            capacity += ingredients[i].capacity * dist[i] as i32;
            durability += ingredients[i].durability * dist[i] as i32;
            flavor += ingredients[i].flavor * dist[i] as i32;
            texture += ingredients[i].texture * dist[i] as i32;
            calories += ingredients[i].calories * dist[i] as i32;
        }

        if capacity > 0
            && durability > 0
            && flavor > 0
            && texture > 0
            && calories == 500
            && (capacity * durability * flavor * texture) as usize > max
        {
            max = (capacity * durability * flavor * texture) as usize;
        }
    }

    max
}
