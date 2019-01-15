// https://adventofcode.com/2015/day/20

pub fn day20(filename: Option<&str>) -> (String, String) {
    let n: u32 = crate::readline(filename.unwrap_or("input/20"))
        .parse()
        .unwrap();

    // Part 1: sum of divisors, each divisor multiplied by 10.
    let a = (1..)
        .skip_while(|&i| divisor_sum(i) * 10 < n)
        .next()
        .unwrap();

    // For part 2 we take the naive approach. We could compare times.
    let b = (1..)
        .skip_while(|&house| {
            divisors(house)
                .iter()
                .filter(|&elf| house <= elf * 50)
                .map(|&elf| elf * 11)
                .sum::<u32>()
                < n
        })
        .next()
        .unwrap();

    (a.to_string(), b.to_string())
}

// I really don't care if this is faster or slower than the naive
// implementation. I'm going to do it with factorization, following
// the geometric formulas at:
//
//     https://www.geeksforgeeks.org/sum-factors-number
//
// With these and a number's factorization calculating the number of
// presents (sum of divisors) is O(1).
//
// P.S.: I thought we'd also need the _number_ of divisors:
//
//     https://www.math.upenn.edu/~deturck/m170/wk2/numdivisors.html
//
// but multiplying divisor_sum() by 10 is enough.

/// Computes the sum of n’s divisors.
fn divisor_sum(n: u32) -> u32 {
    let fact = primes(n);
    let geom = |&(fac, exp)| {
        // An overflow in pow() occurs with u32, hence the cast.
        let fac = u64::from(fac);
        ((fac.pow(exp + 1) - 1) / (fac - 1)) as u32
    };

    fact.iter().map(geom).product()
}

/// Returns a factorization in the form of [(prime, exponent), ...].
fn primes(mut n: u32) -> Vec<(u32, u32)> {
    let mut ret = Vec::new();

    // Divides x by fac until no longer divisible. Returns
    // x/fac^exp, and pushes (fac, exp) to ret if exp > 0.
    let mut fac = |mut x, fac| {
        let mut exp = 0;
        while x % fac == 0 {
            x /= fac;
            exp += 1;
        }
        if exp > 0 {
            ret.push((fac, exp));
        }
        x
    };

    // Not sure why, but the page recommends checking for 2
    // *before* the sqrt() limit.
    n = fac(n, 2);
    let sq = f64::from(n).sqrt().ceil() as u32;

    for i in 3..=sq {
        n = fac(n, i);
    }

    if n > 1 {
        ret.push((n, 1));
    }

    ret
}

// For part 2. It doesn't matter if the list is not sorted.
fn divisors(n: u32) -> Vec<u32> {
    let sq = f64::from(n).sqrt().ceil() as u32;
    let mut ret = Vec::new();

    for i in 1..=sq {
        if n % i == 0 {
            ret.push(i);
            if n / i != i {
                ret.push(n / i);
            }
        }
    }

    ret
}

mod test {
    use super::*;

    #[test]
    fn test_primes() {
        assert_eq!(primes(10), &[(2, 1), (5, 1)]);
        assert_eq!(primes(27), &[(3, 3)]);
        assert_eq!(primes(91875), &[(3, 1), (5, 4), (7, 2)]);
    }

    #[test]
    fn test_divisor_sum() {
        assert_eq!(divisor_sum(15), 24);
        assert_eq!(divisor_sum(18), 39);
        assert_eq!(divisor_sum(30), 72);
    }
}
