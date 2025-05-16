use std::{
    collections::{BinaryHeap, HashSet},
    hash::Hash,
};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
struct State {
    player_hp: usize,
    player_mana: usize,
    mana_spent: usize,
    shield_timer: usize,
    poison_timer: usize,
    recharge_timer: usize,
    boss_hp: usize,
    boss_damage: usize,
}

impl Hash for State {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.player_hp.hash(state);
        self.player_mana.hash(state);
        self.shield_timer.hash(state);
        self.poison_timer.hash(state);
        self.recharge_timer.hash(state);
        self.boss_hp.hash(state);
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.mana_spent.cmp(&other.mana_spent)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let input = include_str!("../input");

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn get_next(state: &State) -> Vec<State> {
    let mut next: Vec<State> = vec![];

    if state.player_mana >= 53 {
        next.push(State {
            player_mana: state.player_mana - 53,
            mana_spent: state.mana_spent + 53,
            boss_hp: if state.boss_hp <= 4 {
                0
            } else {
                state.boss_hp - 4
            },
            ..*state
        });
    }

    if state.player_mana >= 73 {
        next.push(State {
            player_hp: state.player_hp + 2,
            player_mana: state.player_mana - 73,
            mana_spent: state.mana_spent + 73,
            boss_hp: if state.boss_hp <= 2 {
                0
            } else {
                state.boss_hp - 2
            },
            ..*state
        });
    }

    if state.player_mana >= 113 && state.shield_timer == 0 {
        next.push(State {
            player_mana: state.player_mana - 113,
            mana_spent: state.mana_spent + 113,
            shield_timer: 6,
            ..*state
        });
    }

    if state.player_mana >= 173 && state.poison_timer == 0 {
        next.push(State {
            player_mana: state.player_mana - 173,
            mana_spent: state.mana_spent + 173,
            poison_timer: 6,
            ..*state
        });
    }

    if state.player_mana >= 229 && state.recharge_timer == 0 {
        next.push(State {
            player_mana: state.player_mana - 229,
            mana_spent: state.mana_spent + 229,
            recharge_timer: 5,
            ..*state
        })
    }

    next
}

fn part1(input: &str) -> usize {
    let (boss_hp, boss_damage) = input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .last()
                .unwrap()
                .parse::<usize>()
                .unwrap()
        })
        .collect::<Vec<usize>>()
        .windows(2)
        .map(|w| (w[0], w[1]))
        .collect::<Vec<(usize, usize)>>()[0];

    let initial = State {
        player_hp: 50,
        player_mana: 500,
        mana_spent: 0,
        shield_timer: 0,
        poison_timer: 0,
        recharge_timer: 0,
        boss_hp,
        boss_damage,
    };

    let mut queue: BinaryHeap<State> = BinaryHeap::new();
    queue.push(initial);

    let mut seen: HashSet<State> = HashSet::new();
    seen.insert(initial);

    let mut min_mana = usize::MAX;

    while let Some(current) = queue.pop() {
        for next in get_next(&current)
            .into_iter()
            .map(|s| State {
                player_hp: s.player_hp.saturating_sub(
                    1.max(
                        s.boss_damage
                            .saturating_sub(if s.shield_timer > 1 { 7 } else { 0 }),
                    ),
                ),
                player_mana: {
                    match s.recharge_timer {
                        0 => s.player_mana,
                        1 => s.player_mana + 101,
                        _ => s.player_mana + 202,
                    }
                },
                shield_timer: s.shield_timer.saturating_sub(2),
                poison_timer: s.poison_timer.saturating_sub(2),
                recharge_timer: s.recharge_timer.saturating_sub(2),
                boss_hp: {
                    match s.poison_timer {
                        0 => s.boss_hp,
                        1 => s.boss_hp.saturating_sub(3),
                        _ => s.boss_hp.saturating_sub(6),
                    }
                },
                ..s
            })
            .filter(|s| !seen.contains(s) && (s.player_hp != 0 || s.boss_hp == 0))
            .collect::<Vec<State>>()
        {
            if next.player_hp != 0 && next.boss_hp != 0 {
                queue.push(next);
                seen.insert(next);
            } else if next.boss_hp == 0 && next.mana_spent < min_mana {
                min_mana = next.mana_spent;
            }
        }
    }

    min_mana
}

fn part2(input: &str) -> usize {
    let (boss_hp, boss_damage) = input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .last()
                .unwrap()
                .parse::<usize>()
                .unwrap()
        })
        .collect::<Vec<usize>>()
        .windows(2)
        .map(|w| (w[0], w[1]))
        .collect::<Vec<(usize, usize)>>()[0];

    let initial = State {
        player_hp: 49,
        player_mana: 500,
        mana_spent: 0,
        shield_timer: 0,
        poison_timer: 0,
        recharge_timer: 0,
        boss_hp,
        boss_damage,
    };

    let mut queue: BinaryHeap<State> = BinaryHeap::new();
    queue.push(initial);

    let mut seen: HashSet<State> = HashSet::new();
    seen.insert(initial);

    let mut min_mana = usize::MAX;

    while let Some(current) = queue.pop() {
        for next in get_next(&current)
            .into_iter()
            .map(|s| State {
                player_hp: s.player_hp.saturating_sub(
                    1.max(
                        s.boss_damage
                            .saturating_sub(if s.shield_timer > 1 { 7 } else { 0 }),
                    ) + 1,
                ),
                player_mana: {
                    match s.recharge_timer {
                        0 => s.player_mana,
                        1 => s.player_mana + 101,
                        _ => s.player_mana + 202,
                    }
                },
                shield_timer: s.shield_timer.saturating_sub(2),
                poison_timer: s.poison_timer.saturating_sub(2),
                recharge_timer: s.recharge_timer.saturating_sub(2),
                boss_hp: {
                    match s.poison_timer {
                        0 => s.boss_hp,
                        1 => s.boss_hp.saturating_sub(3),
                        _ => s.boss_hp.saturating_sub(6),
                    }
                },
                ..s
            })
            .filter(|s| !seen.contains(s))
            .collect::<Vec<State>>()
        {
            if next.player_hp != 0 && next.boss_hp != 0 {
                queue.push(next);
                seen.insert(next);
            } else if next.boss_hp == 0 && next.mana_spent < min_mana {
                min_mana = next.mana_spent;
            }
        }
    }

    min_mana
}
