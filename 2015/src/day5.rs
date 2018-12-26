// https://adventofcode.com/2015/day/5

use pcre::Pcre; // ¯\_(ツ)_/¯
use std::io::BufRead;

pub fn day5(filename: Option<&str>) -> (String, String) {
    let file = super::bufread(filename.unwrap_or("input/05"));

    let mut a1 = Pcre::compile(r"ab|cd|pq|xy").unwrap();
    let mut a2 = Pcre::compile(r"[aeiou].*[aeiou].*[aeiou]").unwrap();
    let mut a3 = Pcre::compile(r"(.)\1").unwrap();
    let mut b1 = Pcre::compile(r"(..).*\1").unwrap();
    let mut b2 = Pcre::compile(r"(.).\1").unwrap();

    #[rustfmt::skip]
    let mut nice = |l: &str| (a1.exec(l).is_none() &&
                              a2.exec(l).is_some() &&
                              a3.exec(l).is_some());
    #[rustfmt::skip]
    let mut nicer = |l: &str| (b1.exec(l).is_some() &&
                               b2.exec(l).is_some());
    let mut a = 0;
    let mut b = 0;

    for line in file.lines() {
        let l = &line.unwrap();
        if nice(l) {
            a += 1;
        }
        if nicer(l) {
            b += 1;
        }
    }

    (a.to_string(), b.to_string())
}
