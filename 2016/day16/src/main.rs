fn main() {
    let input = include_str!("../input");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> String {
    let mut a = input.trim().to_string();

    while a.len() < 272 {
        let mut new = a.clone();
        new.push('0');
        new.push_str(
            &a.chars()
                .rev()
                .map(|c| if c == '0' { '1' } else { '0' })
                .collect::<String>(),
        );
        a = new;
    }

    let mut checksum = a[..272].to_string();
    while checksum.len() % 2 == 0 {
        checksum = checksum
            .chars()
            .collect::<Vec<char>>()
            .chunks(2)
            .map(|ch| if ch[0] == ch[1] { '1' } else { '0' })
            .collect::<String>();
    }

    checksum
}

fn part2(input: &str) -> String {
    let mut a = input.trim().to_string();

    while a.len() < 35651584 {
        let mut new = a.clone();
        new.push('0');
        new.push_str(
            &a.chars()
                .rev()
                .map(|c| if c == '0' { '1' } else { '0' })
                .collect::<String>(),
        );
        a = new;
    }

    let mut checksum = a[..35651584].to_string();
    while checksum.len() % 2 == 0 {
        checksum = checksum
            .chars()
            .collect::<Vec<char>>()
            .chunks(2)
            .map(|ch| if ch[0] == ch[1] { '1' } else { '0' })
            .collect::<String>();
    }

    checksum
}
