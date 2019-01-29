// https://adventofcode.com/2015/day/8

use regex::Regex;
use std::io::BufRead;

pub fn day8(filename: Option<&str>) -> (String, String) {
    let lines: Vec<_> = crate::bufread(filename.unwrap_or("input/08"))
        .lines()
        .map(|line| line.unwrap())
        .collect();

    // Part 1.
    let seqs = Regex::new(r#"\\x[[:xdigit:]]{2}|\\"|\\\\"#).unwrap();
    let a = &lines
        .iter()
        .map(|line| {
            // We calculate the amount of characters that escaping sequences
            // substract from the original string. Since all escapes reduce to
            // one character, the amount is the length of the sequence - 1. To
            // that value we add 2 to account for the opening and closing quotes.
            2 + seqs
                .find_iter(line)
                .map(|m| m.end() - m.start() - 1)
                .sum::<usize>()
        })
        .sum::<usize>();

    // Part 2.
    let b = lines
        .iter()
        .map(|line| {
            // Here each to-be-escaped character just increases total length by
            // one. We still have to add 2 because (as per AoC) the newly-escaped
            // string ends up wrapped in an extra pair of quotes when serialized.
            2 + line.matches(|c| c == '\\' || c == '\"').count()
        })
        .sum::<usize>();

    (a.to_string(), b.to_string())
}
