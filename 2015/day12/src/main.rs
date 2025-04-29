use serde_json::Value;

fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

fn skip_red(value: &Value) -> i32 {
    match value {
        Value::Object(obj) => {
            if obj.iter().any(|e| e.1 == "red") {
                return 0;
            }

            obj.iter().map(|e| skip_red(e.1)).sum()
        }
        Value::Array(arr) => arr.iter().map(skip_red).sum(),
        Value::Number(n) => n.as_i64().unwrap() as i32,
        _ => 0,
    }
}

fn part1() -> i32 {
    std::fs::read_to_string("input")
        .unwrap()
        .split(['[', ']', '{', '}', '"', ',', ':'])
        .filter_map(|s| s.parse::<i32>().ok())
        .sum()
}

fn part2() -> i32 {
    let object: Value =
        serde_json::from_str(std::fs::read_to_string("input").unwrap().as_str()).unwrap();

    skip_red(&object)
}
