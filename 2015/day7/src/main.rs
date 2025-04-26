use std::collections::HashMap;

#[derive(Debug)]
enum Gate<'a> {
    And {
        left: &'a str,
        right: &'a str,
        output: &'a str,
    },
    Or {
        left: &'a str,
        right: &'a str,
        output: &'a str,
    },
    LShift {
        input: &'a str,
        bits: u16,
        output: &'a str,
    },
    RShift {
        input: &'a str,
        bits: u16,
        output: &'a str,
    },
    Not {
        input: &'a str,
        output: &'a str,
    },
    Equals {
        input: &'a str,
        output: &'a str,
    },
}

impl<'a> Gate<'a> {
    pub fn executed(&self, wires: &HashMap<&str, Wire>) -> bool {
        match self {
            Self::And { output, .. }
            | Self::Or { output, .. }
            | Self::LShift { output, .. }
            | Self::RShift { output, .. }
            | Self::Not { output, .. }
            | Self::Equals { output, .. } => wires.get(output).unwrap().set,
        }
    }

    pub fn execute<'b>(&self, wires: &'b mut HashMap<&'a str, Wire>) {
        if self.executed(wires) {
            return;
        }

        match self {
            Self::And {
                left,
                right,
                output,
            } => {
                if !wires.get(left).unwrap().set || !wires.get(right).unwrap().set {
                    return;
                }

                let temp = wires.get(left).unwrap().value & wires.get(right).unwrap().value;
                wires.get_mut(output).unwrap().set(temp);
            }
            Self::Or {
                left,
                right,
                output,
            } => {
                if !wires.get(left).unwrap().set || !wires.get(right).unwrap().set {
                    return;
                }

                let temp = wires.get(left).unwrap().value | wires.get(right).unwrap().value;
                wires.get_mut(output).unwrap().set(temp);
            }
            Self::LShift {
                input,
                bits,
                output,
            } => {
                if !wires.get(input).unwrap().set {
                    return;
                }

                let temp = wires.get(input).unwrap().value << *bits;
                wires.get_mut(output).unwrap().set(temp);
            }
            Self::RShift {
                input,
                bits,
                output,
            } => {
                if !wires.get(input).unwrap().set {
                    return;
                }

                let temp = wires.get(input).unwrap().value >> *bits;
                wires.get_mut(output).unwrap().set(temp);
            }
            Self::Not { input, output } => {
                if !wires.get(input).unwrap().set {
                    return;
                }

                let temp = !wires.get(input).unwrap().value;
                wires.get_mut(output).unwrap().set(temp);
            }
            Self::Equals { input, output } => {
                if !wires.get(input).unwrap().set {
                    return;
                }

                let temp = wires.get(input).unwrap().value;
                wires.get_mut(output).unwrap().set(temp);
            }
        }
    }
}

#[derive(Default, Debug)]
struct Wire {
    set: bool,
    value: u16,
}

impl Wire {
    pub fn set(&mut self, value: u16) {
        self.set = true;
        self.value = value;
    }
}

fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

fn part1() -> u16 {
    let mut wires: HashMap<&str, Wire> = HashMap::new();
    let mut gates: Vec<Gate> = vec![];

    wires.insert(
        "one",
        Wire {
            set: true,
            value: 1,
        },
    );

    let binding = std::fs::read_to_string("input").unwrap();
    for l in binding.lines().map(|l| l.trim()) {
        let instr = l.split_whitespace().collect::<Vec<&str>>();

        if instr.len() == 3 {
            if !wires.contains_key(instr[2]) {
                wires.insert(instr[2], Wire::default());
            }

            if let Ok(x) = instr[0].parse::<u16>() {
                wires.get_mut(instr[2]).unwrap().set(x);
            } else {
                if !wires.contains_key(instr[0]) {
                    wires.insert(instr[0], Wire::default());
                }

                gates.push(Gate::Equals {
                    input: instr[0],
                    output: instr[2],
                });
            }
        } else if instr.len() == 4 {
            if !wires.contains_key(instr[1]) {
                wires.insert(instr[1], Wire::default());
            }
            if !wires.contains_key(instr[3]) {
                wires.insert(instr[3], Wire::default());
            }

            gates.push(Gate::Not {
                input: instr[1],
                output: instr[3],
            });
        } else if instr[1] == "AND" {
            if !wires.contains_key(instr[2]) {
                wires.insert(instr[2], Wire::default());
            }
            if !wires.contains_key(instr[4]) {
                wires.insert(instr[4], Wire::default());
            }

            if instr[0] == "1" {
                gates.push(Gate::And {
                    left: "one",
                    right: instr[2],
                    output: instr[4],
                });
            } else {
                if !wires.contains_key(instr[0]) {
                    wires.insert(instr[0], Wire::default());
                }

                gates.push(Gate::And {
                    left: instr[0],
                    right: instr[2],
                    output: instr[4],
                });
            }
        } else if instr[1] == "OR" {
            if !wires.contains_key(instr[0]) {
                wires.insert(instr[0], Wire::default());
            }
            if !wires.contains_key(instr[2]) {
                wires.insert(instr[2], Wire::default());
            }
            if !wires.contains_key(instr[4]) {
                wires.insert(instr[4], Wire::default());
            }

            gates.push(Gate::Or {
                left: instr[0],
                right: instr[2],
                output: instr[4],
            });
        } else if instr[1] == "LSHIFT" {
            if !wires.contains_key(instr[0]) {
                wires.insert(instr[0], Wire::default());
            }
            if !wires.contains_key(instr[4]) {
                wires.insert(instr[4], Wire::default());
            }

            gates.push(Gate::LShift {
                input: instr[0],
                bits: instr[2].parse().unwrap(),
                output: instr[4],
            });
        } else if instr[1] == "RSHIFT" {
            if !wires.contains_key(instr[0]) {
                wires.insert(instr[0], Wire::default());
            }
            if !wires.contains_key(instr[4]) {
                wires.insert(instr[4], Wire::default());
            }

            gates.push(Gate::RShift {
                input: instr[0],
                bits: instr[2].parse().unwrap(),
                output: instr[4],
            });
        }
    }

    let mut check = gates.iter().filter(|g| !g.executed(&wires)).count() > 0;
    while check {
        for g in gates.iter_mut() {
            g.execute(&mut wires);
        }
        check = gates.iter().filter(|g| !g.executed(&wires)).count() > 0;
    }

    wires.get("a").unwrap().value
}

fn part2() -> u16 {
    let mut wires: HashMap<&str, Wire> = HashMap::new();
    let mut gates: Vec<Gate> = vec![];

    wires.insert(
        "one",
        Wire {
            set: true,
            value: 1,
        },
    );

    wires.insert(
        "b",
        Wire {
            set: true,
            value: part1(),
        },
    );

    let binding = std::fs::read_to_string("input").unwrap();
    for l in binding.lines().map(|l| l.trim()) {
        let instr = l.split_whitespace().collect::<Vec<&str>>();

        if instr.len() == 3 {
            if !wires.contains_key(instr[2]) {
                wires.insert(instr[2], Wire::default());
            }

            if let Ok(x) = instr[0].parse::<u16>() {
                if instr[2] != "b" {
                    wires.get_mut(instr[2]).unwrap().set(x);
                }
            } else if instr[2] != "b" {
                if !wires.contains_key(instr[0]) {
                    wires.insert(instr[0], Wire::default());
                }

                gates.push(Gate::Equals {
                    input: instr[0],
                    output: instr[2],
                });
            }
        } else if instr.len() == 4 {
            if !wires.contains_key(instr[1]) {
                wires.insert(instr[1], Wire::default());
            }
            if !wires.contains_key(instr[3]) {
                wires.insert(instr[3], Wire::default());
            }

            gates.push(Gate::Not {
                input: instr[1],
                output: instr[3],
            });
        } else if instr[1] == "AND" {
            if !wires.contains_key(instr[2]) {
                wires.insert(instr[2], Wire::default());
            }
            if !wires.contains_key(instr[4]) {
                wires.insert(instr[4], Wire::default());
            }

            if instr[0] == "1" {
                gates.push(Gate::And {
                    left: "one",
                    right: instr[2],
                    output: instr[4],
                });
            } else {
                if !wires.contains_key(instr[0]) {
                    wires.insert(instr[0], Wire::default());
                }

                gates.push(Gate::And {
                    left: instr[0],
                    right: instr[2],
                    output: instr[4],
                });
            }
        } else if instr[1] == "OR" {
            if !wires.contains_key(instr[0]) {
                wires.insert(instr[0], Wire::default());
            }
            if !wires.contains_key(instr[2]) {
                wires.insert(instr[2], Wire::default());
            }
            if !wires.contains_key(instr[4]) {
                wires.insert(instr[4], Wire::default());
            }

            gates.push(Gate::Or {
                left: instr[0],
                right: instr[2],
                output: instr[4],
            });
        } else if instr[1] == "LSHIFT" {
            if !wires.contains_key(instr[0]) {
                wires.insert(instr[0], Wire::default());
            }
            if !wires.contains_key(instr[4]) {
                wires.insert(instr[4], Wire::default());
            }

            gates.push(Gate::LShift {
                input: instr[0],
                bits: instr[2].parse().unwrap(),
                output: instr[4],
            });
        } else if instr[1] == "RSHIFT" {
            if !wires.contains_key(instr[0]) {
                wires.insert(instr[0], Wire::default());
            }
            if !wires.contains_key(instr[4]) {
                wires.insert(instr[4], Wire::default());
            }

            gates.push(Gate::RShift {
                input: instr[0],
                bits: instr[2].parse().unwrap(),
                output: instr[4],
            });
        }
    }

    let mut check = gates.iter().filter(|g| !g.executed(&wires)).count() > 0;
    while check {
        for g in gates.iter_mut() {
            g.execute(&mut wires);
        }
        check = gates.iter().filter(|g| !g.executed(&wires)).count() > 0;
    }

    wires.get("a").unwrap().value
}
