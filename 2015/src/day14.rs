// https://adventofcode.com/2015/day/14

use std::cmp::{max, min};
use std::io::BufRead;

struct Reindeer {
    _name: String,
    speed: u32,
    time_on: u32,
    time_off: u32,
}

struct ScoringReindeer {
    r: Reindeer,
    kms: u32,
    ticks: u32,
    points: u32,
}

impl Reindeer {
    fn travelled_distance(&self, secs: u32) -> u32 {
        let cycle = self.time_on + self.time_off;
        let cycles = secs / cycle;
        let remaining = secs % cycle;

        self.speed * (cycles * self.time_on + min(remaining, self.time_on))
    }
}

impl ScoringReindeer {
    /// Adds a second, updating kilometers travelled so far.
    fn tick(&mut self) {
        self.ticks += 1;
        let cycle = self.r.time_on + self.r.time_off;
        let current = self.ticks % cycle;

        if current > 0 && current <= self.r.time_on {
            self.kms += self.r.speed;
        }
    }

    /// Adds one point for this reindeer.
    fn point(&mut self) {
        self.points += 1;
    }
}

pub fn day14(filename: Option<&str>) -> (String, String) {
    // Part 1: just arithmetic on each Reindeer.
    let vec: Vec<_> = crate::bufread(filename.unwrap_or("input/14")).lines()
        .map(|line| {
            let line = line.unwrap();
            let (_name, speed, time_on, time_off);
            scan!(line.bytes() => "{} can fly {} km/s for {} seconds, but then must rest for {} seconds.",
                  _name, speed, time_on, time_off);
            Reindeer {_name, speed, time_on, time_off}
        })
        .collect();

    let kms = vec
        .iter()
        .map(|r| r.travelled_distance(2503))
        .max()
        .unwrap();

    // Part 2: need to keep track of each step.
    let mut vec: Vec<_> = vec
        .into_iter()
        .map(|r| ScoringReindeer {
            r,
            kms: 0,
            ticks: 0,
            points: 0,
        })
        .collect();

    for _ in 0..2503 {
        let mut best = 0;

        // Advance.
        for r in vec.iter_mut() {
            r.tick();
            best = max(best, r.kms);
        }

        // Score.
        for r in vec.iter_mut() {
            if r.kms == best {
                r.point();
            }
        }
    }

    let points = vec.iter().map(|r| r.points).max().unwrap();

    (kms.to_string(), points.to_string())
}
