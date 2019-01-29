// https://adventofcode.com/2015/day/7 (lifetimes ahoy)

use regex::Regex;
use std::collections::HashMap;
use std::io::BufRead;

#[derive(Debug, Clone)]
enum Op<'a> {
    Int(i32),
    Wire(&'a str),
    Not(&'a str),
    Or(&'a str, &'a str),
    And(&'a str, &'a str),
    LShift(&'a str, u8),
    RShift(&'a str, u8),
}

fn solve<'a>(ops: &'a HashMap<&str, Op>, cache: &mut HashMap<&'a str, i32>, op: &'a str) -> i32 {
    if let Ok(i) = op.parse() {
        return i; // Horrible, but catches integer literals as operands.
    }
    if !cache.contains_key(op) {
        let result = solve_op(ops, cache, &ops[op]);
        cache.insert(op, result);
    }
    cache[op]
}

fn solve_op<'a>(ops: &'a HashMap<&str, Op>, cache: &mut HashMap<&'a str, i32>, op: &'a Op) -> i32 {
    match *op {
        Op::Int(i) => i,
        Op::Not(x) => !solve(ops, cache, x),
        Op::Or(x, y) => solve(ops, cache, x) | solve(ops, cache, y),
        Op::And(x, y) => solve(ops, cache, x) & solve(ops, cache, y),
        Op::LShift(x, y) => solve(ops, cache, x) << y,
        Op::RShift(x, y) => solve(ops, cache, x) >> y,
        Op::Wire(x) => x.parse().unwrap_or_else(|_| solve_op(ops, cache, &ops[x])),
    }
}

pub fn day7(filename: Option<&str>) -> (String, String) {
    let regex = Regex::new(
        r"^((?P<src1>[a-z0-9]+) )?((?P<op>[A-Z]+) )?(?P<src2>[a-z0-9]+) -> (?P<dst>\S+)$",
    )
    .unwrap();
    let lines: Vec<_> = crate::bufread(filename.unwrap_or("input/07"))
        .lines()
        .map(|line| line.unwrap())
        .collect();
    let mut ops = HashMap::new();
    let mut cache = HashMap::new();

    for line in &lines {
        let cap = regex.captures(line).unwrap();
        let grp = |x| cap.name(x).map_or("", |x| x.as_str());
        let dst = grp("dst");
        let src1 = grp("src1");
        let src2 = grp("src2");
        let op = match grp("op") {
            "NOT" => Op::Not(src2),
            "OR" => Op::Or(src1, src2),
            "AND" => Op::And(src1, src2),
            "LSHIFT" => Op::LShift(src1, src2.parse().unwrap()),
            "RSHIFT" => Op::RShift(src1, src2.parse().unwrap()),
            "" => src2.parse().map(|n| Op::Int(n)).unwrap_or(Op::Wire(src2)),
            x => panic!("unknown op: {:?}", x),
        };
        if ops.contains_key(dst) {
            let x = &ops[dst];
            panic!("Duplicate op for {}: had {:?}, now {:?}", dst, x, op);
        }
        ops.insert(dst, op);
    }

    // Part 1.
    let a = solve(&ops, &mut cache, "a");

    // Part 2.
    ops.insert("b", Op::Int(a));
    let mut cache = HashMap::new(); // :-(
    let b = solve(&ops, &mut cache, "a");

    (a.to_string(), b.to_string())
}
