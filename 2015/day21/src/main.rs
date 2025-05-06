use itertools::Itertools;

#[derive(Default, Clone, Copy)]
struct Item {
    cost: usize,
    damage: usize,
    armor: usize,
}

fn main() {
    let input = include_str!("../input");

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn player_wins(
    mut player_hp: usize,
    player_damage: usize,
    player_armor: usize,
    mut boss_hp: usize,
    boss_damage: usize,
    boss_armor: usize,
) -> bool {
    loop {
        if boss_armor >= player_damage {
            boss_hp -= 1;
        } else if player_damage - boss_armor >= boss_hp {
            boss_hp = 0;
        } else {
            boss_hp -= player_damage - boss_armor;
        }

        if boss_hp == 0 {
            return true;
        }

        if player_armor >= boss_damage {
            player_hp -= 1;
        } else if boss_damage - player_armor >= player_hp {
            player_hp = 0;
        } else {
            player_hp -= boss_damage - player_armor;
        }

        if player_hp == 0 {
            return false;
        }
    }
}

fn part1(input: &str) -> usize {
    let (boss_hp, boss_damage, boss_armor) = input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .last()
                .unwrap()
                .parse::<usize>()
                .unwrap()
        })
        .collect::<Vec<usize>>()
        .windows(3)
        .map(|w| (w[0], w[1], w[2]))
        .collect::<Vec<(usize, usize, usize)>>()[0];

    let mut min_cost = usize::MAX;
    for weapon in [
        Item {
            cost: 8,
            damage: 4,
            ..Default::default()
        },
        Item {
            cost: 10,
            damage: 5,
            ..Default::default()
        },
        Item {
            cost: 25,
            damage: 6,
            ..Default::default()
        },
        Item {
            cost: 40,
            damage: 7,
            ..Default::default()
        },
        Item {
            cost: 74,
            damage: 8,
            ..Default::default()
        },
    ] {
        for armor in [
            Item::default(),
            Item {
                cost: 13,
                armor: 1,
                ..Default::default()
            },
            Item {
                cost: 31,
                armor: 2,
                ..Default::default()
            },
            Item {
                cost: 53,
                armor: 3,
                ..Default::default()
            },
            Item {
                cost: 75,
                armor: 4,
                ..Default::default()
            },
            Item {
                cost: 102,
                armor: 5,
                ..Default::default()
            },
        ] {
            for rings in [
                Item {
                    cost: 25,
                    damage: 1,
                    ..Default::default()
                },
                Item {
                    cost: 50,
                    damage: 2,
                    ..Default::default()
                },
                Item {
                    cost: 100,
                    damage: 3,
                    ..Default::default()
                },
                Item {
                    cost: 20,
                    armor: 1,
                    ..Default::default()
                },
                Item {
                    cost: 40,
                    armor: 2,
                    ..Default::default()
                },
                Item {
                    cost: 80,
                    armor: 3,
                    ..Default::default()
                },
            ]
            .iter()
            .powerset()
            .filter(|r| r.len() < 3)
            {
                let items = [
                    weapon,
                    armor,
                    **rings.first().unwrap_or(&&Item::default()),
                    **rings.get(1).unwrap_or(&&Item::default()),
                ];
                if items.iter().map(|item| item.cost).sum::<usize>() < min_cost
                    && player_wins(
                        100,
                        items.iter().map(|item| item.damage).sum::<usize>(),
                        items.iter().map(|item| item.armor).sum(),
                        boss_hp,
                        boss_damage,
                        boss_armor,
                    )
                {
                    min_cost = items.iter().map(|item| item.cost).sum::<usize>();
                }
            }
        }
    }

    min_cost
}

fn part2(input: &str) -> usize {
    let (boss_hp, boss_damage, boss_armor) = input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .last()
                .unwrap()
                .parse::<usize>()
                .unwrap()
        })
        .collect::<Vec<usize>>()
        .windows(3)
        .map(|w| (w[0], w[1], w[2]))
        .collect::<Vec<(usize, usize, usize)>>()[0];

    let mut max_cost = 0;
    for weapon in [
        Item {
            cost: 8,
            damage: 4,
            ..Default::default()
        },
        Item {
            cost: 10,
            damage: 5,
            ..Default::default()
        },
        Item {
            cost: 25,
            damage: 6,
            ..Default::default()
        },
        Item {
            cost: 40,
            damage: 7,
            ..Default::default()
        },
        Item {
            cost: 74,
            damage: 8,
            ..Default::default()
        },
    ] {
        for armor in [
            Item::default(),
            Item {
                cost: 13,
                armor: 1,
                ..Default::default()
            },
            Item {
                cost: 31,
                armor: 2,
                ..Default::default()
            },
            Item {
                cost: 53,
                armor: 3,
                ..Default::default()
            },
            Item {
                cost: 75,
                armor: 4,
                ..Default::default()
            },
            Item {
                cost: 102,
                armor: 5,
                ..Default::default()
            },
        ] {
            for rings in [
                Item {
                    cost: 25,
                    damage: 1,
                    ..Default::default()
                },
                Item {
                    cost: 50,
                    damage: 2,
                    ..Default::default()
                },
                Item {
                    cost: 100,
                    damage: 3,
                    ..Default::default()
                },
                Item {
                    cost: 20,
                    armor: 1,
                    ..Default::default()
                },
                Item {
                    cost: 40,
                    armor: 2,
                    ..Default::default()
                },
                Item {
                    cost: 80,
                    armor: 3,
                    ..Default::default()
                },
            ]
            .iter()
            .powerset()
            .filter(|r| r.len() < 3)
            {
                let items = [
                    weapon,
                    armor,
                    **rings.first().unwrap_or(&&Item::default()),
                    **rings.last().unwrap_or(&&Item::default()),
                ];
                if items.iter().map(|item| item.cost).sum::<usize>() > max_cost
                    && !player_wins(
                        100,
                        items.iter().map(|item| item.damage).sum(),
                        items.iter().map(|item| item.armor).sum(),
                        boss_hp,
                        boss_damage,
                        boss_armor,
                    )
                {
                    max_cost = items.iter().map(|item| item.cost).sum();
                }
            }
        }
    }

    max_cost
}
