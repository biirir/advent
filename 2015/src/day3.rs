// https://adventofcode.com/2015/day/3

use std::collections::HashMap;

type Pos = (i32, i32);

pub fn day3() -> (String, String) {
    let line = super::readline("input/03");

    // Part 1
    let mut pos = (0, 0);
    let mut map = HashMap::new();
    map.insert(pos, ());

    for dir in line.chars() {
        pos = advance(dir, pos);
        map.insert(pos, ());
    }

    // Part 2
    let mut newpos = [(0, 0); 2];
    let mut newmap: HashMap<Pos, ()> = HashMap::new();
    map.insert(pos, ());

    for (i, dir) in line.chars().enumerate() {
        let pos = &mut newpos[i % 2];
        *pos = advance(dir, *pos);
        newmap.insert(*pos, ());
    }

    (map.len().to_string(), newmap.len().to_string())
}

fn advance(c: char, (x, y): Pos) -> Pos {
    match c {
        '>' => (x + 1, y),
        '<' => (x - 1, y),
        '^' => (x, y + 1),
        'v' => (x, y - 1),
        _ => (x, y),
    }
}
