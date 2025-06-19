fn main() {
    let input = include_str!("../input");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let mut current = input.trim().chars().collect::<Vec<char>>();

    while current
        .windows(2)
        .any(|w| w[1] as u8 == w[0] as u8 - 32 || w[1] as u8 == w[0] as u8 + 32)
    {
        let mut temp: Vec<char> = vec![];
        let mut i = 0;
        while i < current.len() - 1 {
            if current[i + 1] as u8 == current[i] as u8 - 32
                || current[i + 1] as u8 == current[i] as u8 + 32
            {
                i += 2;
            } else {
                temp.push(current[i]);
                i += 1;
            }
        }
        if temp[temp.len() - 1] == current[current.len() - 2] {
            temp.push(current[current.len() - 1]);
        }
        current = temp;
    }

    current.len()
}

fn part2(input: &str) -> usize {
    let mut min = usize::MAX;

    for c in 'A'..='Z' {
        min = min.min(part1(
            &input
                .chars()
                .filter(|ch| *ch != c && *ch != (c as u8 + 32) as char)
                .collect::<String>(),
        ));
    }

    min
}
