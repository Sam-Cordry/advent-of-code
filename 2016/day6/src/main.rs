use std::collections::HashMap;

fn main() {
    let input = include_str!("../input");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> String {
    let mut result: HashMap<usize, HashMap<char, usize>> = HashMap::new();

    for l in input.lines().map(|l| l.trim()) {
        for (i, c) in l.char_indices() {
            result
                .entry(i)
                .and_modify(|f| {
                    f.entry(c).and_modify(|c| *c += 1).or_insert(1);
                })
                .or_insert(HashMap::from([(c, 1)]));
        }
    }

    let mut temp = result
        .iter()
        .map(|e| (*e.0, e.1))
        .collect::<Vec<(usize, &HashMap<char, usize>)>>();
    temp.sort_by(|a, b| a.0.cmp(&b.0));
    temp.iter()
        .map(|(_, hm)| hm.iter().max_by(|a, b| a.1.cmp(b.1)).unwrap().0)
        .collect::<String>()
}

fn part2(input: &str) -> String {
    let mut result: HashMap<usize, HashMap<char, usize>> = HashMap::new();

    for l in input.lines().map(|l| l.trim()) {
        for (i, c) in l.char_indices() {
            result
                .entry(i)
                .and_modify(|f| {
                    f.entry(c).and_modify(|c| *c += 1).or_insert(1);
                })
                .or_insert(HashMap::from([(c, 1)]));
        }
    }

    let mut temp = result
        .iter()
        .map(|e| (*e.0, e.1))
        .collect::<Vec<(usize, &HashMap<char, usize>)>>();
    temp.sort_by(|a, b| a.0.cmp(&b.0));
    temp.iter()
        .map(|(_, hm)| hm.iter().min_by(|a, b| a.1.cmp(b.1)).unwrap().0)
        .collect::<String>()
}
