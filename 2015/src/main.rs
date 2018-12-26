mod day1;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

type DayFunc = fn() -> (String, String);

static FUNCS: &'static [DayFunc] = &[day1::day1];

fn bufread(filename: &str) -> BufReader<File> {
    BufReader::new(File::open(filename).unwrap())
}

fn readline(filename: &str) -> String {
    bufread(filename).lines().next().unwrap().unwrap()
}

fn main() {
    let argv: Vec<_> = env::args().collect();
    let mut day = FUNCS.len();

    if argv.len() > 1 {
        day = argv[1].parse().expect("not a number");
    }

    let (a, b) = FUNCS[day - 1]();

    println!("Day {}, part 1: {}", day, a);
    println!("Day {}, part 2: {}", day, b);
}

#[cfg(test)]
mod tests {
    #[test]
    fn day1() {
        let (a, b) = super::day1::day1();
        assert_eq!("232", a);
        assert_eq!("1783", b);
    }
}
