// https://adventofcode.com/2015/day/13

use std::cmp;
use std::collections::HashMap;
use std::io::BufRead;

use super::graph::Graph;

fn next_permutation<T: PartialOrd>(perm: &mut [T]) -> bool {
    // https://en.wikipedia.org/wiki/Permutation#Generation_in_lexicographic_order
    let len = perm.len();
    let k = 'K: loop {
        for k in (0..len - 1).rev() {
            if perm[k] < perm[k + 1] {
                break 'K k;
            }
        }
        return false;
    };
    let l = 'L: loop {
        for l in (k + 1..len).rev() {
            if perm[k] < perm[l] {
                break 'L l;
            }
        }
    };
    perm.swap(k, l);
    perm[k + 1..].reverse();
    true
}

fn optimal_happiness(g: &Graph) -> i32 {
    let mut vec: Vec<_>;
    let mut best = std::i32::MIN;
    let setup;

    let happiness = |xs: &[usize]| {
        let mut h = 0;
        let len = xs.len();
        for i in 0..len {
            // For each guest V...
            let v = xs[i];
            // determine their neighbours: left L, right R.
            let l = xs[(i + 1) % len];
            let r = xs[(i + len - 1) % len];
            // For each happiness delta V declares...
            for (w, e) in &g.adj[v] {
                let w = *w as usize;
                // include it if it's about about L or R.
                if w == l || w == r {
                    h += e.weight;
                }
            }
        }
        h
    };

    vec = (0..g.V()).collect();
    setup = vec.as_mut_slice();

    loop {
        best = cmp::max(best, happiness(setup));
        if !next_permutation(setup) {
            break best;
        }
    }
}

pub fn day13(filename: Option<&str>) -> (String, String) {
    let mut graph = Graph::new_directed();
    let mut guests: HashMap<String, u32> = HashMap::new();

    // Map city names to a unique vertex number.
    let mut guest = |g| {
        let len = guests.len();
        *guests.entry(g).or_insert(len as u32)
    };

    for line in super::bufread(filename.unwrap_or("input/13")).lines() {
        let (who, with);
        let act: String;
        let mut amt: i32;
        let line = line.unwrap();
        scan!(line.bytes() => "{} would {} {} happiness units by sitting next to {}.",
              who, act, amt, with);
        if act == "lose" {
            amt = -amt;
        }
        graph.add_edge(guest(who), guest(with), amt);
    }

    // Part 1.
    let a = optimal_happiness(&graph);

    // Part 2: an extra guest with 0 delta.
    let b;
    let u = graph.V() as u32;

    for v in 0..u {
        graph.add_edge(u, v, 0);
        graph.add_edge(v, u, 0);
    }
    b = optimal_happiness(&graph);

    (a.to_string(), b.to_string())
}
