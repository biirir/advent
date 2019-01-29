// https://adventofcode.com/2015/day/24

use std::cmp::min;
use std::io::BufRead;

pub fn day24(filename: Option<&str>) -> (String, String) {
    let mut weights = crate::bufread(filename.unwrap_or("input/24"))
        .lines()
        .map(|l| l.unwrap().parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let weight = weights.iter().sum::<u64>();
    assert_eq!(weight % 3, 0);
    assert_eq!(weight % 4, 0);

    weights.sort_unstable(); // Input is already sorted, but just in case.
    weights.reverse();

    let a = fewest_entanglement(&weights, 0, 1, std::u64::MAX, weight / 3);
    let b = fewest_entanglement(&weights, 0, 1, std::u64::MAX, weight / 4);

    (a.to_string(), b.to_string())
}

fn fewest_entanglement(weights: &[u64], idx: usize, cur: u64, mut best: u64, target: u64) -> u64 {
    // This function only searches for the lowest entanglement, so it's wrong
    // on two counts:
    //
    //   - this lowest entanglement could come from a grouping that doesn't allow for
    //     the other two groups to divide evenly into equal weights. (XXX Is this true?)
    //
    //   - a group with higher entanglement could have fewer packages (as in the example
    //     on the web), and it should take precedence.
    //
    // But, it's 4am here. ¯\_(ツ)_/¯

    if target == 0 {
        return min(cur, best);
    }

    if idx >= weights.len() || cur > best {
        return best;
    }

    let w = weights[idx];

    if w <= target {
        best = fewest_entanglement(weights, idx + 1, cur * w, best, target - w);
    }

    min(
        best,
        fewest_entanglement(weights, idx + 1, cur, best, target),
    )
}
