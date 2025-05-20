fn main() {
    let input = include_str!("../input");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let mut decompressed_len = 0;

    let chars = input.trim().chars().collect::<Vec<char>>();
    let mut idx = 0;
    while idx < chars.len() {
        if chars[idx] != '(' {
            decompressed_len += 1;
            idx += 1;
            continue;
        }

        let num = chars[idx + 1..chars.iter().skip(idx).position(|c| *c == 'x').unwrap() + idx]
            .iter()
            .collect::<String>()
            .parse::<usize>()
            .unwrap();
        let times = chars[chars.iter().skip(idx).position(|c| *c == 'x').unwrap() + idx + 1
            ..chars.iter().skip(idx).position(|c| *c == ')').unwrap() + idx]
            .iter()
            .collect::<String>()
            .parse::<usize>()
            .unwrap();

        decompressed_len += num * times;
        idx = chars.iter().skip(idx).position(|c| *c == ')').unwrap() + idx + num + 1;
    }

    decompressed_len
}

fn decompression_len_rec(s: &str) -> usize {
    if !s.contains('(') {
        s.len()
    } else {
        let mut result = 0;

        let chars = s.chars().collect::<Vec<char>>();
        let mut idx = 0;
        while idx < chars.len() {
            if chars[idx] == '(' {
                let num = chars
                    [idx + 1..chars.iter().skip(idx).position(|c| *c == 'x').unwrap() + idx]
                    .iter()
                    .collect::<String>()
                    .parse::<usize>()
                    .unwrap();
                let times = chars[chars.iter().skip(idx).position(|c| *c == 'x').unwrap() + idx + 1
                    ..chars.iter().skip(idx).position(|c| *c == ')').unwrap() + idx]
                    .iter()
                    .collect::<String>()
                    .parse::<usize>()
                    .unwrap();

                let start = chars.iter().skip(idx).position(|c| *c == ')').unwrap() + idx + 1;
                result +=
                    decompression_len_rec(&chars[start..start + num].iter().collect::<String>())
                        * times;

                idx = start + num;
            } else {
                idx += 1;
                result += 1;
            }
        }

        result
    }
}

fn part2(input: &str) -> usize {
    decompression_len_rec(input.trim())
}
