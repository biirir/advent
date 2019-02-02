// https://adventofcode.com/2015/day/22

use lazy_static::lazy_static;
use maplit::hashmap;
use simple_error::bail;

use std::cmp::{max, min};
use std::collections::HashMap;
use std::fs;

pub fn day22(filename: Option<&str>) -> (String, String) {
    let input = fs::read_to_string(filename.unwrap_or("input/22")).unwrap();
    let (points, damage);
    scan!(input.bytes() => "Hit Points: {}\nDamage: {}\n", points, damage);

    let boss = Boss { points, damage };
    let player = Wizard::new(50, 500);
    let min_easy = min_mana(player, boss, 0, std::u32::MAX);

    (min_easy.to_string(), "".to_owned())
}

fn min_mana(in_player: Wizard, in_boss: Boss, cur: u32, mut best: u32) -> u32 {
    for spell in SPELLS_BY_COST {
        let newcur = cur + COSTS[spell];
        if newcur > best {
            break;
        }
        let mut boss = in_boss.clone();
        let mut player = in_player.clone();
        match player.advance_turn(&mut boss, *spell) {
            Ok(Outcome::Win) => return min(best, newcur),
            Ok(Outcome::Continue) => {
                best = min(best, min_mana(player, boss, newcur, best));
            }
            Err(Failure::NoMana) => break,
            _ => continue,
        }
    }
    best
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Spell {
    Missile,
    Drain,
    Shield,
    Poison,
    Recharge,
}

lazy_static! {
    static ref COSTS: HashMap<Spell, u32> = hashmap! {
        Spell::Missile => 53,
        Spell::Drain => 73,
        Spell::Shield => 113,
        Spell::Poison => 173,
        Spell::Recharge => 229,
    };
}

const SPELLS_BY_COST: &[Spell] = &[
    Spell::Missile,
    Spell::Drain,
    Spell::Shield,
    Spell::Poison,
    Spell::Recharge,
];

#[derive(Debug)]
enum Outcome {
    Win,      // Boss dead.
    Lose,     // Player dead.
    Continue, // Both alive.
}

#[derive(Debug)]
enum Failure {
    NoMana,
    DuplicateSpell,
}

#[derive(Clone)]
struct Wizard {
    mana: u32,
    armor: i32,
    points: i32,
    effects: Vec<(i32, Spell)>,
}

#[derive(Clone)]
struct Boss {
    points: i32,
    damage: i32,
}

impl Wizard {
    fn new(points: i32, mana: u32) -> Wizard {
        Wizard {
            mana,
            points,
            armor: 0,
            effects: Vec::new(),
        }
    }

    // Plays our spell, and then the boss's turn.
    fn advance_turn(&mut self, boss: &mut Boss, spell: Spell) -> Result<Outcome, Failure> {
        // eprintln!("-- Player turn --");
        // eprintln!(
        //     "- Player has {} hit points, {} armor, {} mana",
        //     self.points, self.armor, self.mana
        // );
        // eprintln!("- Boss has {} hit points", boss.points);

        //
        // Check it's not a duplicate spell.
        //
        for (count, curspell) in &self.effects {
            if *curspell == spell && *count > 0 {
                bail!(Failure::DuplicateSpell);
            }
        }

        //
        // Apply active effects.
        //
        self.apply_effects(boss);

        if boss.points <= 0 {
            return Ok(Outcome::Win);
        } else if self.points <= 0 {
            return Ok(Outcome::Lose);
        }

        //
        // Pay for the spell, or return error.
        //
        if COSTS[&spell] <= self.mana {
            self.mana -= COSTS[&spell];
        } else {
            bail!(Failure::NoMana);
        }

        //
        // Perform the spell.
        //
        // eprintln!("Player casts {:?}", spell);
        match spell {
            Spell::Missile => {
                boss.points -= 4;
            }
            Spell::Drain => {
                boss.points -= 2;
                self.points += 2;
            }
            Spell::Shield => {
                self.effects.push((6, spell));
            }
            Spell::Poison => {
                self.effects.push((6, spell));
            }
            Spell::Recharge => {
                self.effects.push((5, spell));
            }
        };

        if boss.points <= 0 {
            return Ok(Outcome::Win);
        } else if self.points <= 0 {
            return Ok(Outcome::Lose);
        }

        //
        // Boss' turn.
        //
        // eprintln!("-- Boss turn --");
        // eprintln!(
        //     "- Player has {} hit points, {} armor, {} mana",
        //     self.points, self.armor, self.mana
        // );
        // eprintln!("- Boss has {} hit points", boss.points);

        //
        // Apply effects again.
        //
        self.apply_effects(boss);

        if boss.points <= 0 {
            return Ok(Outcome::Win);
        } else if self.points <= 0 {
            return Ok(Outcome::Lose);
        }

        let damage = max(1, boss.damage - self.armor);
        self.points -= damage;

        // eprintln!("Boss attacks for {} damage", damage);

        if boss.points <= 0 {
            return Ok(Outcome::Win);
        } else if self.points <= 0 {
            return Ok(Outcome::Lose);
        }

        Ok(Outcome::Continue)
    }

    fn apply_effects(&mut self, boss: &mut Boss) {
        for (count, spell) in self.effects.iter_mut() {
            assert!(*count >= 0);
            if *count > 0 {
                // eprintln!("Applying effect {:?}, remaining={}", spell, *count - 1);
            } else {
                // eprintln!("Deactivating {:?}", spell);
            }
            match *spell {
                Spell::Poison => {
                    if *count > 0 {
                        boss.points -= 3;
                    }
                }
                Spell::Recharge => {
                    if *count > 0 {
                        self.mana += 101;
                    }
                }
                Spell::Shield => {
                    if *count == 6 {
                        self.armor += 7;
                    } else if *count == 0 {
                        self.armor -= 7;
                    }
                }
                _ => panic!("Spell {:?} has no associated effect", spell),
            }
            *count -= 1;
        }
        self.effects.retain(|&(count, _)| count >= 0);
    }
}
