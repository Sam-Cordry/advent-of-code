fn main() {
    let input = include_str!("../input");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let mut list = (0..256).collect::<Vec<usize>>();
    let mut idx = 0;

    for (skip, len) in input
        .split(',')
        .map(|s| s.trim().parse::<usize>().unwrap())
        .enumerate()
    {
        let mut new = list.split_off(idx);
        new.append(&mut list);

        let mut temp = new.split_off(len);
        new.reverse();
        new.append(&mut temp);

        list = new.split_off(new.len() - idx);
        list.append(&mut new);

        idx += len + skip;
        idx %= list.len();
    }

    list[0] * list[1]
}

fn part2(input: &str) -> String {
    let mut len = input
        .trim()
        .as_bytes()
        .to_vec()
        .iter()
        .map(|&n| n as usize)
        .collect::<Vec<usize>>();
    len.extend([17, 31, 73, 47, 23]);

    let mut list = (0..256).collect::<Vec<usize>>();
    let mut idx = 0;
    let mut skip = 0;

    for _ in 0..64 {
        for l in len.iter() {
            let mut new = list.split_off(idx);
            new.append(&mut list);

            let mut temp = new.split_off(*l);
            new.reverse();
            new.append(&mut temp);

            list = new.split_off(new.len() - idx);
            list.append(&mut new);

            idx += l + skip;
            idx %= list.len();
            skip += 1;
        }
    }

    let hex = list
        .chunks(16)
        .map(|c| c.iter().fold(0, |agg, n| agg ^ n) as u8)
        .collect::<Vec<u8>>();

    let mut result = String::new();
    for h in hex {
        result.push_str(&format!("{h:02x}"));
    }
    result
}
