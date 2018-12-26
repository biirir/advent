#[macro_use]
extern crate text_io;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

type DayFunc = fn(Option<&str>) -> (String, String);

static FUNCS: &'static [DayFunc] = &[day1::day1, day2::day2, day3::day3, day4::day4, day5::day5];

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
}
