use std::collections::HashMap;

fn main() {
    let input = include_str!("../input");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let mut result = 0;

    for room in input.lines().map(|l| l.trim()) {
        let mut frequency: HashMap<char, usize> = HashMap::new();
        for c in room[..room.rfind('-').unwrap()].chars() {
            if c == '-' {
                continue;
            }

            frequency.entry(c).and_modify(|c| *c += 1).or_insert(1);
        }

        let mut temp = frequency
            .iter()
            .map(|e| (e.0, e.1))
            .collect::<Vec<(&char, &usize)>>();
        temp.sort_by(|a, b| {
            if a.1 != b.1 {
                b.1.cmp(a.1)
            } else {
                a.0.cmp(b.0)
            }
        });

        if room[room.find('[').unwrap() + 1..room.find(']').unwrap()]
            == temp.iter().take(5).map(|e| *e.0).collect::<String>()
        {
            result += room[room.rfind('-').unwrap() + 1..room.find('[').unwrap()]
                .parse::<usize>()
                .unwrap();
        }
    }

    result
}

fn part2(input: &str) -> usize {
    for room in input.lines().map(|l| l.trim()) {
        let mut frequency: HashMap<char, usize> = HashMap::new();
        for c in room[..room.rfind('-').unwrap()].chars() {
            if c == '-' {
                continue;
            }

            frequency.entry(c).and_modify(|c| *c += 1).or_insert(1);
        }

        let mut temp = frequency
            .iter()
            .map(|e| (e.0, e.1))
            .collect::<Vec<(&char, &usize)>>();
        temp.sort_by(|a, b| {
            if a.1 != b.1 {
                b.1.cmp(a.1)
            } else {
                a.0.cmp(b.0)
            }
        });

        if room[room.find('[').unwrap() + 1..room.find(']').unwrap()]
            == temp.iter().take(5).map(|e| *e.0).collect::<String>()
        {
            let shift = room[room.rfind('-').unwrap() + 1..room.find('[').unwrap()]
                .parse::<usize>()
                .unwrap();
            let mut new = String::new();
            for c in room[..room.rfind('-').unwrap()].chars() {
                if c == '-' {
                    new.push(' ');
                } else {
                    new.push(((c as usize - 97 + shift) % 26 + 97) as u8 as char);
                }
            }
            if new == "northpole object storage" {
                return shift;
            }
        }
    }

    0
}
