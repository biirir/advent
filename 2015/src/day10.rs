// https://adventofcode.com/2015/day/10

use itertools::Itertools;

fn iter(string: &str, times: i32) -> String {
    let mut digits = string.to_owned();

    // Used with Itertools::coalesce() to count contiguous runs of characters.
    let contiguous = |(n, x), (m, y)| {
        if x == y {
            Ok((n + m, x))
        } else {
            Err(((n, x), (m, y)))
        }
    };

    for _ in 0..times {
        let mut s = String::new();
        for (n, c) in digits.chars().map(|c| (1, c)).coalesce(contiguous) {
            match n {
                0...9 => {
                    // Cast to char to avoid an allocation.
                    let d = (n as u8) + b'0';
                    s.push(d as char);
                }
                _ => {
                    s.push_str(&n.to_string());
                }
            }
            s.push(c);
        }
        digits = s;
    }
    digits
}

pub fn day10(filename: Option<&str>) -> (String, String) {
    let input = super::readline(filename.unwrap_or("input/10"));
    let one = iter(&input, 40);
    let two = iter(&input, 50);

    (one.len().to_string(), two.len().to_string())
}
