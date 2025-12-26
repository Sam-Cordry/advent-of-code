type JunctionBox = (usize, usize, usize);

fn main() {
    let input = include_str!("../input");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn euclidean_distance(a: (usize, usize, usize), b: (usize, usize, usize)) -> f32 {
    ((a.0.abs_diff(b.0).pow(2) + a.1.abs_diff(b.1).pow(2) + a.2.abs_diff(b.2).pow(2)) as f32).sqrt()
}

fn find_min(
    boxes: &[JunctionBox],
    pairs: &[(JunctionBox, JunctionBox)],
) -> (JunctionBox, JunctionBox) {
    let mut min = f32::MAX;
    let mut values: (JunctionBox, JunctionBox) = ((0, 0, 0), (0, 0, 0));

    for i in 0..(boxes.len() - 1) {
        for j in (i + 1)..boxes.len() {
            if euclidean_distance(boxes[i], boxes[j]) < min
                && !pairs.contains(&(boxes[i], boxes[j]))
            {
                min = euclidean_distance(boxes[i], boxes[j]);
                values = (boxes[i], boxes[j]);
            }
        }
    }

    values
}

fn part1(input: &str) -> usize {
    let boxes: Vec<JunctionBox> = input
        .lines()
        .map(|l| {
            let split: Vec<usize> = l.split(',').map(|s| s.parse::<usize>().unwrap()).collect();
            (split[0], split[1], split[2])
        })
        .collect();
    let mut pairs: Vec<(JunctionBox, JunctionBox)> = vec![];
    let mut circuits: Vec<Vec<JunctionBox>> = boxes.iter().map(|b| vec![*b]).collect();

    for _ in 0..1000 {
        let (a, b) = find_min(&boxes, &pairs);
        pairs.push((a, b));

        if !circuits.iter().any(|c| c.contains(&a) && c.contains(&b)) {
            let mut copy = circuits.remove(circuits.iter().position(|c| c.contains(&b)).unwrap());
            circuits
                .iter_mut()
                .find(|c| c.contains(&a))
                .unwrap()
                .append(&mut copy);
        }
    }

    let mut sizes: Vec<usize> = circuits.iter().map(|c| c.len()).collect();
    sizes.sort_by(|a, b| b.cmp(a));
    sizes.iter().take(3).product()
}

fn part2(input: &str) -> usize {
    let boxes: Vec<JunctionBox> = input
        .lines()
        .map(|l| {
            let split: Vec<usize> = l.split(',').map(|s| s.parse::<usize>().unwrap()).collect();
            (split[0], split[1], split[2])
        })
        .collect();
    let mut pairs: Vec<(JunctionBox, JunctionBox)> = vec![];
    let mut circuits: Vec<Vec<JunctionBox>> = boxes.iter().map(|b| vec![*b]).collect();

    loop {
        let (a, b) = find_min(&boxes, &pairs);
        pairs.push((a, b));

        if !circuits.iter().any(|c| c.contains(&a) && c.contains(&b)) {
            let mut copy = circuits.remove(circuits.iter().position(|c| c.contains(&b)).unwrap());
            circuits
                .iter_mut()
                .find(|c| c.contains(&a))
                .unwrap()
                .append(&mut copy);

            if circuits.len() == 1 {
                return a.0 * b.0;
            }
        }
    }
}
