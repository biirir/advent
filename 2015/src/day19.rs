// https://adventofcode.com/2015/day/19

use itertools::Itertools;
use regex::Regex;

use std::collections::{HashMap, HashSet};
use std::io::BufRead;

pub fn day19(filename: Option<&str>) -> (String, String) {
    let input = crate::bufread(filename.unwrap_or("input/19"));
    let mut last = false;
    let mut molecule = String::new();
    let mut transform: HashMap<String, Vec<String>> = HashMap::new();

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

    (derive_count.to_string(), "".to_owned())
}
