use itertools::Itertools;
use std::collections::HashMap;

fn main() {
    let input = include_str!("../input");

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> i32 {
    let mut people: HashMap<&str, HashMap<&str, i32>> = HashMap::new();

    for line in input
        .lines()
        .map(|l| l.split_whitespace().collect::<Vec<&str>>())
    {
        if !people.contains_key(line[0]) {
            people.insert(line[0], HashMap::new());
        }

        people.get_mut(line[0]).unwrap().insert(
            &line[10][..line[10].len() - 1],
            line[3].parse::<i32>().unwrap() * if line[2] == "lose" { -1 } else { 1 },
        );
    }

    let mut max: i32 = 0;
    for perm in people.keys().permutations(people.len()) {
        let mut sum: i32 = 0;
        for i in 0..perm.len() - 1 {
            sum += people.get(perm[i]).unwrap().get(perm[i + 1]).unwrap();
            sum += people.get(perm[i + 1]).unwrap().get(perm[i]).unwrap();
        }
        sum += people
            .get(perm[0])
            .unwrap()
            .get(perm[perm.len() - 1])
            .unwrap();
        sum += people
            .get(perm[perm.len() - 1])
            .unwrap()
            .get(perm[0])
            .unwrap();

        if sum > max {
            max = sum;
        }
    }

    max
}

fn part2(input: &str) -> i32 {
    let mut people: HashMap<&str, HashMap<&str, i32>> = HashMap::new();

    for line in input
        .lines()
        .map(|l| l.split_whitespace().collect::<Vec<&str>>())
    {
        if !people.contains_key(line[0]) {
            people.insert(line[0], HashMap::new());
        }

        people.get_mut(line[0]).unwrap().insert(
            &line[10][..line[10].len() - 1],
            line[3].parse::<i32>().unwrap() * if line[2] == "lose" { -1 } else { 1 },
        );
    }

    people.insert("Me", HashMap::new());

    for person in people.clone().keys() {
        people.get_mut(person).unwrap().insert("Me", 0);
        people.get_mut("Me").unwrap().insert(person, 0);
    }

    let mut max: i32 = 0;
    for perm in people.keys().permutations(people.len()) {
        let mut sum: i32 = 0;
        for i in 0..perm.len() - 1 {
            sum += people.get(perm[i]).unwrap().get(perm[i + 1]).unwrap();
            sum += people.get(perm[i + 1]).unwrap().get(perm[i]).unwrap();
        }
        sum += people
            .get(perm[0])
            .unwrap()
            .get(perm[perm.len() - 1])
            .unwrap();
        sum += people
            .get(perm[perm.len() - 1])
            .unwrap()
            .get(perm[0])
            .unwrap();

        if sum > max {
            max = sum;
        }
    }

    max
}
