use std::collections::HashMap;

#[derive(PartialEq, Eq)]
enum Action {
    Begin,
    Sleep,
    Wake,
}

#[derive(PartialEq, Eq)]
struct Timestamp {
    year: usize,
    month: usize,
    day: usize,
    hour: usize,
    minute: usize,
}

impl PartialOrd for Timestamp {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Timestamp {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.year != other.year {
            self.year.cmp(&other.year)
        } else if self.month != other.month {
            self.month.cmp(&other.month)
        } else if self.day != other.day {
            self.day.cmp(&other.day)
        } else if self.hour != other.hour {
            self.hour.cmp(&other.hour)
        } else {
            self.minute.cmp(&other.minute)
        }
    }
}

#[derive(PartialEq, Eq)]
struct Record {
    guard: Option<usize>,
    timestamp: Timestamp,
    action: Action,
}

impl PartialOrd for Record {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Record {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.timestamp.cmp(&other.timestamp)
    }
}

fn main() {
    let input = include_str!("../input");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let mut records: Vec<Record> = vec![];

    for l in input.lines() {
        records.push(Record {
            guard: if l.contains("Guard") {
                Some(
                    l.chars()
                        .skip(26)
                        .take_while(|c| *c != ' ')
                        .collect::<String>()
                        .parse()
                        .unwrap(),
                )
            } else {
                None
            },
            timestamp: Timestamp {
                year: l[1..5].parse().unwrap(),
                month: l[6..8].parse().unwrap(),
                day: l[9..11].parse().unwrap(),
                hour: l[12..14].parse().unwrap(),
                minute: l[15..17].parse().unwrap(),
            },
            action: if l.contains("Guard") {
                Action::Begin
            } else if l.contains("falls") {
                Action::Sleep
            } else {
                Action::Wake
            },
        });
    }

    records.sort();

    let mut asleep: HashMap<usize, usize> = HashMap::new();
    let mut guard = 0;

    for i in 0..records.len() {
        if records[i].action == Action::Begin {
            guard = records[i].guard.unwrap();
        } else if records[i].action == Action::Sleep {
            asleep
                .entry(guard)
                .and_modify(|v| *v += records[i + 1].timestamp.minute - records[i].timestamp.minute)
                .or_insert(records[i + 1].timestamp.minute - records[i].timestamp.minute);
        }
    }

    let max_guard = *asleep.iter().max_by(|a, b| a.1.cmp(b.1)).unwrap().0;
    let mut minutes = [0; 60];
    guard = 0;

    for i in 0..records.len() {
        if records[i].action == Action::Begin {
            guard = records[i].guard.unwrap();
        } else if records[i].action == Action::Sleep && guard == max_guard {
            for m in minutes
                .iter_mut()
                .take(records[i + 1].timestamp.minute)
                .skip(records[i].timestamp.minute)
            {
                *m += 1;
            }
        }
    }

    let max_min = *minutes.iter().max().unwrap();
    max_guard * minutes.iter().position(|m| *m == max_min).unwrap()
}

fn part2(input: &str) -> usize {
    let mut records: Vec<Record> = vec![];

    for l in input.lines() {
        records.push(Record {
            guard: if l.contains("Guard") {
                Some(
                    l.chars()
                        .skip(26)
                        .take_while(|c| *c != ' ')
                        .collect::<String>()
                        .parse()
                        .unwrap(),
                )
            } else {
                None
            },
            timestamp: Timestamp {
                year: l[1..5].parse().unwrap(),
                month: l[6..8].parse().unwrap(),
                day: l[9..11].parse().unwrap(),
                hour: l[12..14].parse().unwrap(),
                minute: l[15..17].parse().unwrap(),
            },
            action: if l.contains("Guard") {
                Action::Begin
            } else if l.contains("falls") {
                Action::Sleep
            } else {
                Action::Wake
            },
        });
    }

    records.sort();

    let mut minutes: Vec<HashMap<usize, usize>> = vec![HashMap::new(); 60];
    let mut guard = 0;

    for i in 0..records.len() {
        if records[i].action == Action::Begin {
            guard = records[i].guard.unwrap();
        } else if records[i].action == Action::Sleep {
            for m in minutes
                .iter_mut()
                .take(records[i + 1].timestamp.minute)
                .skip(records[i].timestamp.minute)
            {
                m.entry(guard).and_modify(|v| *v += 1).or_insert(1);
            }
        }
    }

    let maxs: Vec<(usize, usize)> = minutes
        .iter()
        .cloned()
        .map(|e| e.into_iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap_or((0, 0)))
        .collect();
    let max_time = maxs.iter().map(|(_, v)| *v).max().unwrap();

    maxs.iter().find(|m| m.1 == max_time).unwrap().0
        * maxs.iter().position(|m| m.1 == max_time).unwrap()
}
