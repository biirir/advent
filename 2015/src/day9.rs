// https://adventofcode.com/2015/day/9 (duplication ahoy)

use std::collections::{HashMap, HashSet};
use std::io::BufRead;

use super::graph::Graph;

type Prune = fn(i32, i32) -> bool;
type Accept = fn(i32, i32) -> bool;
type TspResult = (Option<Vec<u32>>, i32);

struct TSP<'a> {
    g: &'a Graph,
    path: Vec<u32>,
    seen: HashSet<u32>,
    prune: Prune,
    accept: Accept,
}

impl<'a> TSP<'a> {
    pub fn new(g: &'a Graph, prune: Prune, accept: Accept) -> TSP<'a> {
        TSP {
            g,
            prune,
            accept,
            path: Vec::new(),
            seen: HashSet::with_capacity(g.V()),
        }
    }

    pub fn tsp(&mut self) -> TspResult {
        self._tsp(0, 0, (None, std::i32::MAX))
    }
}

pub fn day9(filename: Option<&str>) -> (String, String) {
    let mut graph = Graph::new();
    let mut cities = HashMap::new();

    // Map city names to a unique vertex number.
    let mut vertex = |c| {
        let len = cities.len();
        *cities.entry(c).or_insert(len as u32)
    };

    for line in super::bufread(filename.unwrap_or("input/09")).lines() {
        let a: String;
        let b: String;
        let dist: i32;
        let line = line.unwrap();
        scan!(line.bytes() => "{} to {} = {}", a, b, dist);
        graph.add_edge(vertex(a), vertex(b), dist);
    }

    let (_short, short) = TSP::new(&graph, |c, d| c >= d, |c, d| c < d).tsp();
    let (_long, longer) = TSP::new(&graph, |_, _| false, |c, d| c > d).tsp();

    (short.to_string(), longer.to_string())
}

impl<'a> TSP<'a> {
    fn _tsp(&mut self, cur: u32, cost: i32, mut best: TspResult) -> TspResult {
        if (self.prune)(cost, best.1) {
            return best;
        }

        if self.seen.len() == self.g.V() - 1 {
            if best.0.is_none() || (self.accept)(cost, best.1) {
                let mut path = self.path.clone();
                path.push(cur);
                return (Some(path), cost);
            } else {
                return best;
            }
        }

        self.path.push(cur);
        self.seen.insert(cur);

        for (u, e) in &self.g.adj[cur as usize] {
            if self.seen.contains(u) {
                continue;
            }
            best = self._tsp(*u, cost + e.weight, best);
        }

        self.path.pop();
        self.seen.remove(&cur);

        if self.path.is_empty() && (cur as usize) < self.g.V() - 1 {
            best = self._tsp(cur + 1, 0, best);
        }

        best
    }
}
