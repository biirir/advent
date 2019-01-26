// https://adventofcode.com/2015/day/17

use std::io::BufRead;

pub fn day17(filename: Option<&str>) -> (String, String) {
    let input = crate::bufread(filename.unwrap_or("input/17"));
    let target = 150;
    let mut containers = Vec::new();

    for line in input.lines() {
        containers.push(line.unwrap().parse::<u32>().unwrap());
    }

    containers.sort_unstable();
    let all = all_combinations(&containers, 0, target);
    let mut best = (std::u32::MAX, 0);
    min_combinations(&containers, 0, 0, target, &mut best);

    (all.to_string(), best.1.to_string())
}

// Part 1.
fn all_combinations(sizes: &[u32], cur: usize, target: u32) -> u32 {
    let with_us;

    if cur >= sizes.len() || sizes[cur] > target {
        // If sizes is sorted, there are no possible combinations left.
        return 0;
    } else if sizes[cur] == target {
        // Just by ourselves we can complete the ongoing set.
        with_us = 1;
    } else {
        // We can't complete the set fully on our own (sizes[cur] < target).
        with_us = all_combinations(sizes, cur + 1, target - sizes[cur]);
    }

    // Return what we achieved, plus whatever they will achieve without us.
    with_us + all_combinations(sizes, cur + 1, target)
}

// Part 2.
type Best = (u32, u32);

fn min_combinations(sizes: &[u32], cur_idx: usize, cur_size: u32, target: u32, best: &mut Best) {
    let (ref mut best_size, ref mut best_count) = best;

    if cur_size > *best_size || cur_idx >= sizes.len() || sizes[cur_idx] > target {
        // No possible combinations left (sizes is sorted).
        return;
    } else if sizes[cur_idx] == target {
        // Just by ourselves we can complete the ongoing set, *and* we might
        // have a new minimum, *or* we might have matched the current one.
        if cur_size + 1 < *best_size {
            *best_size = cur_size + 1;
            *best_count = 1;
        } else if cur_size + 1 == *best_size {
            *best_count += 1;
        }
    } else {
        // Let's continue, first adding ourselves to the ongoing set.
        #[rustfmt::skip]
        min_combinations(sizes, cur_idx + 1, cur_size + 1, target - sizes[cur_idx], best);
    }

    // Combinations without us must be always considered.
    min_combinations(sizes, cur_idx + 1, cur_size, target, best);
}
