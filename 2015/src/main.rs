#[macro_use]
extern crate text_io;

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day18;
mod day2;
mod day20;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod graph;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

type DayFunc = fn(Option<&str>) -> (String, String);

#[rustfmt::skip]
static FUNCS: &'static [DayFunc] = &[
    day1::day1, day2::day2, day3::day3, day4::day4, day5::day5, day6::day6,
    day7::day7, day8::day8, day9::day9, day10::day10, day11::day11, day12::day12,
    day13::day13, day14::day14, day15::day15, day18::day18,
    day20::day20,
];

fn bufread(filename: &str) -> BufReader<File> {
    BufReader::new(File::open(filename).unwrap())
}

fn readline(filename: &str) -> String {
    bufread(filename).lines().next().unwrap().unwrap()
}

fn main() {
    let argv: Vec<_> = env::args().collect();
    let mut day = FUNCS.len();
    let mut file: Option<&str> = None;

    if argv.len() > 1 {
        day = argv[1].parse().expect("not a number");
    }

    if argv.len() > 2 {
        file = Some(&argv[2]);
    }

    let (a, b) = FUNCS[day - 1](file);

    println!("Day {}, part 1: {}", day, a);
    println!("Day {}, part 2: {}", day, b);
}

#[cfg(test)]
mod tests {
    #[test]
    fn day1() {
        let (a, b) = super::day1::day1(None);
        assert_eq!("232", a);
        assert_eq!("1783", b);
    }

    #[test]
    fn day2() {
        let (a, b) = super::day2::day2(None);
        assert_eq!("1586300", a);
        assert_eq!("3737498", b);
    }

    #[test]
    fn day3() {
        let (a, b) = super::day3::day3(None);
        assert_eq!("2565", a);
        assert_eq!("2639", b);
    }

    #[test]
    fn day4() {
        let (a, b) = super::day4::day4(None);
        assert_eq!("254575", a);
        assert_eq!("1038736", b);
    }

    #[test]
    fn day5() {
        let (a, b) = super::day5::day5(None);
        assert_eq!("236", a);
        assert_eq!("51", b);
    }

    #[test]
    fn day6() {
        let (a, b) = super::day6::day6(None);
        assert_eq!("543903", a);
        assert_eq!("14687245", b);
    }

    #[test]
    fn day7() {
        let (a, b) = super::day7::day7(None);
        assert_eq!("956", a);
        assert_eq!("40149", b);
    }

    #[test]
    fn day8() {
        let (a, b) = super::day8::day8(None);
        assert_eq!("1333", a);
        assert_eq!("2046", b);
    }

    #[test]
    fn day9() {
        let (a, b) = super::day9::day9(None);
        assert_eq!("141", a);
        assert_eq!("736", b);
    }

    #[test]
    fn day10() {
        let (a, b) = super::day10::day10(None);
        assert_eq!("329356", a);
        assert_eq!("4666278", b);
    }

    #[test]
    fn day11() {
        let (a, b) = super::day11::day11(None);
        assert_eq!("hxbxxyzz", a);
        assert_eq!("hxcaabcc", b);
    }

    #[test]
    fn day12() {
        let (a, b) = super::day12::day12(None);
        assert_eq!("111754", a);
        assert_eq!("65402", b);
    }

    #[test]
    fn day13() {
        let (a, b) = super::day13::day13(None);
        assert_eq!("664", a);
        assert_eq!("640", b);
    }

    #[test]
    fn day14() {
        let (a, b) = super::day14::day14(None);
        assert_eq!("2640", a);
        assert_eq!("1102", b);
    }

    #[test]
    fn day15() {
        let (a, b) = super::day15::day15(None);
        assert_eq!("222870", a);
        assert_eq!("117936", b);
    }

    #[test]
    fn day18() {
        let (a, b) = super::day18::day18(None);
        assert_eq!("821", a);
        assert_eq!("886", b);
    }

    #[test]
    #[ignore] // Slow.
    fn day20() {
        let (a, b) = super::day20::day20(None);
        assert_eq!("786240", a);
        assert_eq!("831600", b);
    }
}
