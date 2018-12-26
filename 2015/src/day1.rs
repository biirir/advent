// https://adventofcode.com/2015/day/1
pub fn day1() -> (String, String) {
    let brackint = |c: char| match c {
        '(' => 1,
        ')' => -1,
        _ => 0,
    };
    let line = super::readline("input/01");
    let mut floor = 0;
    let mut basement = 0;

    for (i, c) in line.chars().enumerate() {
        floor += brackint(c);
        if floor == -1 && basement == 0 {
            basement = i + 1
        }
    }

    (floor.to_string(), basement.to_string())
}
