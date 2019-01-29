// https://adventofcode.com/2015/day/21

use std::cmp::{max, min};
use std::fs;

use self::weapons::*;
use crate::ceildiv;

struct Player {
    points: u32,
    damage: i32,
    armor: i32,
}

#[rustfmt::skip]
pub fn day21(filename: Option<&str>) -> (String, String) {
    let input = fs::read_to_string(filename.unwrap_or("input/21")).unwrap();
    let (points, damage, armor);

    scan!(input.bytes() => "Hit Points: {}\nDamage: {}\nArmor: {}", points, damage, armor);
    let boss = Player {points, damage, armor};

    let (best, worst) = best_worst(&boss);
    (best.to_string(), worst.to_string())
}

fn first_wins(first: &Player, second: &Player) -> bool {
    // How much points does each first lose per round.
    let loss1 = max(1, second.damage - first.armor) as u32;
    let loss2 = max(1, first.damage - second.armor) as u32;

    // How many rounds it takes for each first to hit 0 points.
    let rounds1 = ceildiv(first.points, loss1);
    let rounds2 = ceildiv(second.points, loss2);

    rounds1 >= rounds2
}

fn best_worst(boss: &Player) -> (u32, u32) {
    let mut best = std::u32::MAX;
    let mut worst = std::u32::MIN;

    for weapon in WEAPONS {
        for armor in ARMOR {
            for (i, ring1) in RINGS.iter().enumerate() {
                for (j, ring2) in RINGS.iter().enumerate() {
                    if i == j {
                        // Cannot buy same ring twice. (We use enumerate to
                        // avoid implementing Eq on Weapon. Plus there might
                        // be in stock two copies of the very same ring, which
                        // we would be probably allowed to buy.)
                        continue;
                    }
                    let cost = weapon.cost + armor.cost + ring1.cost + ring2.cost;
                    let player = Player {
                        points: 100,
                        damage: weapon.damage + ring1.damage + ring2.damage,
                        armor: armor.armor + ring1.armor + ring2.armor,
                    };
                    if first_wins(&player, boss) {
                        best = min(best, cost);
                    } else {
                        worst = max(worst, cost);
                    }
                }
            }
        }
    }
    (best, worst)
}

// Dump/hide weapon definitions into a module.
mod weapons {
    pub struct Weapon {
        pub cost: u32,
        pub damage: i32,
        pub armor: i32,
    }

    pub const WEAPONS: &[Weapon] = &[
        Weapon {
            // Dagger
            cost: 8,
            damage: 4,
            armor: 0,
        },
        Weapon {
            // Shortsword
            cost: 10,
            damage: 5,
            armor: 0,
        },
        Weapon {
            // Warhammer
            cost: 25,
            damage: 6,
            armor: 0,
        },
        Weapon {
            // Longsword
            cost: 40,
            damage: 7,
            armor: 0,
        },
        Weapon {
            // Greataxe
            cost: 74,
            damage: 8,
            armor: 0,
        },
    ];

    pub const ARMOR: &[Weapon] = &[
        Weapon {
            // No armor.
            cost: 0,
            damage: 0,
            armor: 0,
        },
        Weapon {
            // Leather
            cost: 13,
            damage: 0,
            armor: 1,
        },
        Weapon {
            // Chainmail
            cost: 31,
            damage: 0,
            armor: 2,
        },
        Weapon {
            // Splintmail
            cost: 53,
            damage: 0,
            armor: 3,
        },
        Weapon {
            // Bandedmail
            cost: 75,
            damage: 0,
            armor: 4,
        },
        Weapon {
            // Platemail
            cost: 102,
            damage: 0,
            armor: 5,
        },
    ];

    pub const RINGS: &[Weapon] = &[
        Weapon {
            // No ring.
            cost: 0,
            damage: 0,
            armor: 0,
        },
        Weapon {
            // Damage +1
            cost: 25,
            damage: 1,
            armor: 0,
        },
        Weapon {
            // Damage +2
            cost: 50,
            damage: 2,
            armor: 0,
        },
        Weapon {
            // Damage +3
            cost: 100,
            damage: 3,
            armor: 0,
        },
        Weapon {
            // Defense +1
            cost: 20,
            damage: 0,
            armor: 1,
        },
        Weapon {
            // Defense +2
            cost: 40,
            damage: 0,
            armor: 2,
        },
        Weapon {
            // Defense +3
            cost: 80,
            damage: 0,
            armor: 3,
        },
    ];
}
