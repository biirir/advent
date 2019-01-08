// https://adventofcode.com/2015/day/15

use itertools::Itertools;

use std::cmp::max;
use std::io::BufRead;

struct Ingredient {
    _name: String,
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

struct Mixtures {
    vec: Vec<Vec<i32>>,
}

impl Iterator for Mixtures {
    type Item = Vec<i32>;

    fn next(&mut self) -> Option<Vec<i32>> {
        self.vec.pop()
    }
}

impl Mixtures {
    fn new(len: usize, tot: i32) -> Mixtures {
        let mut mix = Mixtures { vec: Vec::new() };
        mix.generate(len, tot);
        mix
    }

    // Taken from:
    //
    //     https://www.reddit.com/r/adventofcode/comments/3wwj84/day_15_solutions/cxzk44a/
    //
    // except Rust doesn't have generators. /o\
    // We simulate by returning a list of indices of the “yielded”
    // items in self.vec. (Horrible code.)
    fn generate(&mut self, len: usize, tot: i32) -> Vec<usize> {
        let beg = match len {
            1 => tot,
            _ => 0,
        };
        let mut idx = Vec::new();
        for i in beg..=tot {
            let rem = tot - i;
            if len == 1 {
                idx.push(self.vec.len());
                self.vec.push(vec![i]);
            } else {
                for j in self.generate(len - 1, rem).into_iter() {
                    idx.push(j);
                    self.vec[j].push(i as i32);
                }
            }
        }
        return idx;
    }
}

fn score(ingredients: &[Ingredient], amounts: &[i32], exact_calories: Option<i32>) -> i32 {
    // This constructs a vector of 5 elements, with global per-quality scores.
    let mut scores: Vec<_> = ingredients
        .iter()
        .zip(amounts)
        .map(|(t, amt)| {
            // Per-ingredient, per-quality scores.
            vec![
                amt * t.capacity,
                amt * t.durability,
                amt * t.flavor,
                amt * t.texture,
                amt * t.calories,
            ]
        })
        .fold1(|a, b| {
            // Adding up individual per-quality scores.
            a.iter().zip(b).map(|(x, y)| x + y).collect()
        })
        .unwrap_or(vec![0]);

    // Check for calorie requirements.
    let cal = scores.pop().unwrap();

    if let Some(x) = exact_calories {
        if x != cal {
            return 0;
        }
    }

    // Compute global score, killing the group on any negative value.
    scores
        .into_iter()
        .fold(1, |acc, n| if n < 0 { 0 } else { acc * n })
}

pub fn day15(filename: Option<&str>) -> (String, String) {
    let vec: Vec<_> = super::bufread(filename.unwrap_or("input/15")).lines()
        .map(|line| {
            let line = line.unwrap();
            let (_name, capacity, durability, flavor, texture, calories);
            scan!(line.bytes() => "{}: capacity {}, durability {}, flavor {}, texture {}, calories {}",
                  _name, capacity, durability, flavor, texture, calories);
            // vec![capacity, durability, flavor, texture, calories]
            Ingredient {_name, capacity, durability, flavor, texture, calories }
        })
        .collect();

    // Part 1.
    let mut a = 0;
    let mut b = 0;

    for mixt in Mixtures::new(vec.len(), 100) {
        a = max(a, score(&vec, &mixt, None));
        b = max(b, score(&vec, &mixt, Some(500)));
    }

    (a.to_string(), b.to_string())
}
