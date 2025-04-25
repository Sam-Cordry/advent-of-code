use md5::{Digest, Md5};

fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

fn part1() -> usize {
    let mut count: usize = 1;
    let mut hasher = Md5::new();

    let input = std::fs::read_to_string("input")
        .unwrap()
        .trim()
        .as_bytes()
        .to_vec();

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

fn part2() -> usize {
    let mut count: usize = 1;
    let mut hasher = Md5::new();

    let input = std::fs::read_to_string("input")
        .unwrap()
        .trim()
        .as_bytes()
        .to_vec();

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
