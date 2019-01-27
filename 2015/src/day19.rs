// https://adventofcode.com/2015/day/19

use itertools::Itertools;
use regex::Regex;

use std::cmp::min;
use std::collections::{HashMap, HashSet};
use std::io::BufRead;

pub fn day19(filename: Option<&str>) -> (String, String) {
    let input = crate::bufread(filename.unwrap_or("input/19"));
    let mut last = false;
    let mut molecule = String::new();
    let mut transform: HashMap<String, Vec<String>> = HashMap::new();
    let mut reverse_transform = HashMap::new();

    for line in input.lines() {
        let (src, dst);
        let line = line.unwrap();
        if last {
            molecule = line;
        } else if line.is_empty() {
            last = true;
        } else {
            scan!(line.bytes() => "{} => {}", src, dst);
            transform.entry(src).or_default().push(dst);
        }
    }

    for (src, vec) in &transform {
        for dst in vec {
            if let Some(src) = reverse_transform.insert(dst, src) {
                eprintln!("Now don't be lazy, {} is multi-origin", src);
            }
        }
    }

    // So it's really hard to split the molecule on capital letters (and
    // there might be non-uppercase replacements); so we create a regular
    // expression and replace each match.
    let regex = Regex::new(&transform.keys().join("|")).unwrap();
    let mut start = 0;
    let mut molecules = HashSet::new();
    let mut locations = regex.capture_locations();

    // Part 1.
    let derive_count = loop {
        match regex.captures_read_at(&mut locations, &molecule, start) {
            None => break molecules.len(),
            Some(m) => {
                for replacement in &transform[m.as_str()] {
                    let mut molecule = molecule.clone();
                    molecule.replace_range(m.start()..m.end(), replacement);
                    molecules.insert(molecule);
                    start = m.end();
                }
            }
        }
    };

    // Part 2.
    let rev_regex = Regex::new(&reverse_transform.keys().join("|")).unwrap();
    let mut replacement_count = 0;
    let mut pos = molecule.len() - 2;

    while molecule != "e" {
        // Find a match, *from the right*. It's an easy way to work against the
        // grammar of the problem. See these Reddit threads:
        // https://www.reddit.com/r/adventofcode/comments/3xhkeb
        // https://www.reddit.com/r/adventofcode/comments/3xflz8/day_19_solutions/cy4etju
        while pos > 0 && !rev_regex.is_match_at(&molecule, pos) {
            pos -= 1;
        }
        if let Some(m) = rev_regex.find_at(&molecule, pos) {
            let (i, j) = (m.start(), m.end());
            let replacement = m.as_str().to_string();
            molecule.replace_range(i..j, reverse_transform[&replacement]);
            replacement_count += 1;
            pos = min(j, molecule.len() - 1);
        } else {
            println!("Failed to reduce at pos={}, molecule={}.", pos, molecule);
        }
    }

    (derive_count.to_string(), replacement_count.to_string())
}
