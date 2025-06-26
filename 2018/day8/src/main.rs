#[derive(Default)]
struct Node {
    children: Vec<Node>,
    metadata: Vec<usize>,
}

impl Node {
    fn sum_metadata(&self) -> usize {
        self.metadata.iter().sum::<usize>()
            + self
                .children
                .iter()
                .map(|n| n.sum_metadata())
                .sum::<usize>()
    }

    fn get_value(&self) -> usize {
        if self.children.is_empty() {
            self.metadata.iter().sum::<usize>()
        } else {
            let mut sum = 0;

            for m in self.metadata.iter() {
                sum += self.children.get(m - 1).map(|n| n.get_value()).unwrap_or(0);
            }

            sum
        }
    }
}

fn main() {
    let input = include_str!("../input");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn parse_node(data: &[usize]) -> (usize, Node) {
    let mut node = Node::default();
    let mut i = 2;

    for _ in 0..data[0] {
        let result = parse_node(&data[i..]);
        node.children.push(result.1);
        i += result.0;
    }
    for _ in 0..data[1] {
        node.metadata.push(data[i]);
        i += 1;
    }

    (i, node)
}

fn part1(input: &str) -> usize {
    parse_node(
        &input
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect::<Vec<usize>>(),
    )
    .1
    .sum_metadata()
}

fn part2(input: &str) -> usize {
    parse_node(
        &input
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect::<Vec<usize>>(),
    )
    .1
    .get_value()
}
