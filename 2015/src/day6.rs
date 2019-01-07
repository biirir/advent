// https://adventofcode.com/2015/day/6

use regex::Regex;
use std::io::BufRead;

enum Action {
    On,
    Off,
    Toggle,
}

impl Action {
    fn new(s: &str) -> Action {
        match s {
            "turn on" => Action::On,
            "turn off" => Action::Off,
            "toggle" => Action::Toggle,
            _ => panic!("invalid action {:?}", s),
        }
    }

    fn apply(&self, cur: i32) -> i32 {
        match self {
            Action::On => 1,
            Action::Off => 0,
            Action::Toggle => 1 - cur,
        }
    }

    fn apply2(&self, cur: i32) -> i32 {
        match self {
            Action::On => cur + 1,
            Action::Off => std::cmp::max(cur - 1, 0),
            Action::Toggle => cur + 2,
        }
    }
}

pub fn day6(filename: Option<&str>) -> (String, String) {
    let regex: Regex =
        Regex::new(r"(turn on|turn off|toggle) (\d+),(\d+) through (\d+),(\d+)").unwrap();
    let actions: Vec<(Action, usize, usize, usize, usize)> =
        super::bufread(filename.unwrap_or("input/06"))
            .lines()
            .map(|line| {
                let line = line.unwrap();
                let cap = regex.captures(&line).unwrap();
                let action = Action::new(&cap[1]);
                let (a, b, c, d) = (
                    cap[2].parse().unwrap(),
                    cap[3].parse().unwrap(),
                    cap[4].parse().unwrap(),
                    cap[5].parse().unwrap(),
                );
                (action, a, b, c, d)
            })
            .collect();

    // Creating an array this big directly on the stack kills cargo test with
    // stack overflow (because it uses threads, and these have smaller stacks.)
    let mut grid = vec![(0, 0); 1000 * 1000].into_boxed_slice();

    for (action, a, b, c, d) in actions {
        for i in a..=c {
            for j in b..=d {
                let t = &mut grid[i * 1000 + j];
                t.0 = action.apply(t.0);
                t.1 = action.apply2(t.1);
            }
        }
    }

    let result = grid.iter().fold((0, 0), |(n, m), (a, b)| (n + a, m + b));
    (result.0.to_string(), result.1.to_string())
}
