use md5::{Digest, Md5};

fn main() {
    let input = include_str!("../input");

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let mut count: usize = 1;
    let mut hasher = Md5::new();

    let input = input.trim().as_bytes().to_vec();

    loop {
        let mut temp = input.clone();
        temp.extend_from_slice(count.to_string().as_bytes());

        hasher.update(temp);

        if &format!("{:x}", hasher.finalize_reset())[..5] == "00000" {
            break;
        }

        count += 1;
    }

    count
}

fn part2(input: &str) -> usize {
    let mut count: usize = 1;
    let mut hasher = Md5::new();

    let input = input.trim().as_bytes().to_vec();

    loop {
        let mut temp = input.clone();
        temp.extend_from_slice(count.to_string().as_bytes());

        hasher.update(temp);

        if &format!("{:x}", hasher.finalize_reset())[..6] == "000000" {
            break;
        }

        count += 1;
    }

    count
}
