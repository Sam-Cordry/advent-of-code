use std::collections::HashMap;

type Vector = (i32, i32, i32);

fn main() {
    let input = include_str!("../input");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn manhattan_distance(v: Vector) -> usize {
    (v.0.abs() * v.1.abs() * v.1.abs()) as usize
}

fn part1(input: &str) -> usize {
    let mut particles: Vec<(usize, (Vector, Vector, Vector))> = vec![];

    for (i, l) in input.lines().enumerate() {
        particles.push((
            i,
            l.split(['<', '>', ','])
                .filter_map(|s| s.parse::<i32>().ok())
                .collect::<Vec<i32>>()
                .chunks(3)
                .map(|w| (w[0], w[1], w[2]))
                .collect::<Vec<(i32, i32, i32)>>()
                .chunks(3)
                .map(|w| (w[0], w[1], w[2]))
                .next()
                .unwrap(),
        ));
    }

    particles.sort_by(|a, b| manhattan_distance(a.1 .0).cmp(&manhattan_distance(b.1 .0)));
    particles.sort_by(|a, b| manhattan_distance(a.1 .1).cmp(&manhattan_distance(b.1 .1)));
    particles.sort_by(|a, b| manhattan_distance(a.1 .2).cmp(&manhattan_distance(b.1 .2)));

    particles[0].0
}

fn part2(input: &str) -> usize {
    let mut particles: Vec<(usize, (Vector, Vector, Vector))> = vec![];

    for (i, l) in input.lines().enumerate() {
        particles.push((
            i,
            l.split(['<', '>', ','])
                .filter_map(|s| s.parse::<i32>().ok())
                .collect::<Vec<i32>>()
                .chunks(3)
                .map(|w| (w[0], w[1], w[2]))
                .collect::<Vec<(i32, i32, i32)>>()
                .chunks(3)
                .map(|w| (w[0], w[1], w[2]))
                .next()
                .unwrap(),
        ));
    }

    for _ in 0..1000 {
        let mut coords: HashMap<(i32, i32, i32), Vec<usize>> = HashMap::new();

        for p in particles.iter_mut() {
            p.1 .1 .0 += p.1 .2 .0;
            p.1 .1 .1 += p.1 .2 .1;
            p.1 .1 .2 += p.1 .2 .2;
            p.1 .0 .0 += p.1 .1 .0;
            p.1 .0 .1 += p.1 .1 .1;
            p.1 .0 .2 += p.1 .1 .2;

            coords
                .entry((p.1 .0 .0, p.1 .0 .1, p.1 .0 .2))
                .and_modify(|v| v.push(p.0))
                .or_insert(vec![p.0]);
        }

        for dup in coords.values().filter(|v| v.len() > 1).flatten() {
            particles.remove(particles.iter().position(|p| p.0 == *dup).unwrap());
        }
    }

    particles.len()
}
