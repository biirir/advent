// https://adventofcode.com/2015/day/16

use maplit::hashmap;
use regex::Regex;

use std::cmp::Ordering;
use std::collections::HashMap;
use std::io::BufRead;

pub fn day16(filename: Option<&str>) -> (String, String) {
    let input = crate::bufread(filename.unwrap_or("input/16"));
    let regex = Regex::new(r" (?P<category>\S+): (?P<amount>\d+)(?:,|$)").unwrap();

    // Attributes our aunt has.
    let wanted = hashmap! {
        "children" => 3,
        "cats" => 7,
        "samoyeds" => 2,
        "pomeranians" => 3,
        "akitas" => 0,
        "vizslas" => 0,
        "goldfish" => 5,
        "trees" => 3,
        "cars" => 2,
        "perfumes" => 1,
    };

    // For each part, what info does the machine give us (if not present,
    // default is Ordering::Equal).
    let match1 = HashMap::new();
    let match2 = hashmap! {
        "cats" => Ordering::Greater,
        "trees" => Ordering::Greater,
        "pomeranians" => Ordering::Less,
        "goldfish" => Ordering::Less,
    };

    // Whether all attributes of an aunt match the desired ones (as per matcher).
    let is_good = |matcher: &HashMap<&'static str, Ordering>, line: &str| {
        for cap in regex.captures_iter(line) {
            let cat = &cap["category"];
            let num = cap["amount"].parse::<u32>().unwrap();
            let ord = matcher.get(cat).cloned();
            if num.cmp(&wanted[cat]) != ord.unwrap_or(Ordering::Equal) {
                return false;
            }
        }
        true
    };

    let mut aunt1 = 0;
    let mut aunt2 = 0;

    for (i, line) in input.lines().enumerate() {
        let line = line.unwrap();
        if aunt1 == 0 && is_good(&match1, &line) {
            aunt1 = i + 1;
        }
        if aunt2 == 0 && is_good(&match2, &line) {
            aunt2 = i + 1;
        }
        if aunt1 > 0 && aunt2 > 0 {
            break;
        }
    }

    (aunt1.to_string(), aunt2.to_string())
}
