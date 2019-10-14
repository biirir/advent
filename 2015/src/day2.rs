// https://adventofcode.com/2015/day/2
// This line was also changed by Rohith.
use std::cmp::min;
use std::io::BufRead;

struct Piece {
    l: i32,
    w: i32,
    h: i32,
}

impl Piece {
    #[rustfmt::skip]
    fn paper(&self) -> i32 {
        let (a, b, c) = (self.l * self.w,
                         self.w * self.h,
                         self.h * self.l);
        2 * (a + b + c) + min(a, min(b, c))
    }
    #[rustfmt::skip]
    fn ribbon(&self) -> i32 {
        let (p, q, r) = (self.l * 2 + self.w * 2,
                         self.w * 2 + self.h * 2,
                         self.h * 2 + self.l * 2);
        min(p, min(q, r)) + self.l * self.w * self.h
    }
}

pub fn day2(filename: Option<&str>) -> (String, String) {
    let v: Vec<_> = crate::bufread(filename.unwrap_or("input/02"))
        .lines()
        .map(|line| {
            let (l, w, h);
            let line = line.unwrap();
            scan!(line.bytes() => "{}x{}x{}", l, w, h);
            Piece { l, w, h }
        })
        .collect();

    let paper = v.iter().fold(0, |acc, p| acc + p.paper());
    let ribbon = v.iter().fold(0, |acc, p| acc + p.ribbon());

    (paper.to_string(), ribbon.to_string())
}
