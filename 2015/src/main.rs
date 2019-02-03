#[macro_use]
extern crate text_io;

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day2;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod graph;

use num_traits as num;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

type DayFunc = fn(Option<&str>) -> (String, String);

#[rustfmt::skip]
static FUNCS: &'static [DayFunc] = &[
    day1::day1, day2::day2, day3::day3, day4::day4, day5::day5, day6::day6,
    day7::day7, day8::day8, day9::day9, day10::day10, day11::day11, day12::day12,
    day13::day13, day14::day14, day15::day15, day16::day16, day17::day17, day18::day18,
    day19::day19, day20::day20, day21::day21, day22::day22, day23::day23, day24::day24,
    day25::day25,
];

fn bufread(filename: &str) -> BufReader<File> {
    BufReader::new(File::open(filename).unwrap())
}

fn readline(filename: &str) -> String {
    bufread(filename).lines().next().unwrap().unwrap()
}

// Given two integers, computes ⌈n/d⌉.
fn ceildiv<T>(n: T, d: T) -> T
where
    T: num::Unsigned + Copy,
{
    (n + d - T::one()) / d
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
    fn day16() {
        let (a, b) = super::day16::day16(None);
        assert_eq!("213", a);
        assert_eq!("323", b);
    }

    #[test]
    fn day17() {
        let (a, b) = super::day17::day17(None);
        assert_eq!("4372", a);
        assert_eq!("4", b);
    }

    #[test]
    fn day18() {
        let (a, b) = super::day18::day18(None);
        assert_eq!("821", a);
        assert_eq!("886", b);
    }

    #[test]
    fn day19() {
        let (a, b) = super::day19::day19(None);
        assert_eq!("509", a);
        assert_eq!("195", b);
    }

    #[test]
    #[ignore] // Slow.
    fn day20() {
        let (a, b) = super::day20::day20(None);
        assert_eq!("786240", a);
        assert_eq!("831600", b);
    }

    #[test]
    fn day21() {
        let (a, b) = super::day21::day21(None);
        assert_eq!("111", a);
        assert_eq!("188", b);
    }

    #[test]
    fn day22() {
        let (a, b) = super::day22::day22(None);
        assert_eq!("953", a);
        assert_eq!("1289", b);
    }

    #[test]
    fn day23() {
        let (a, b) = super::day23::day23(None);
        assert_eq!("255", a);
        assert_eq!("334", b);
    }

    #[test]
    fn day24() {
        let (a, b) = super::day24::day24(None);
        assert_eq!("10439961859", a);
        assert_eq!("72050269", b);
    }

    #[test]
    fn day25() {
        let (a, b) = super::day25::day25(None);
        assert_eq!("19980801", a);
        assert_eq!("", b);
    }
}
