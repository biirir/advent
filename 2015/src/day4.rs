// https://adventofcode.com/2015/day/4
use md5::Digest;

pub fn day4(input: Option<&str>) -> (String, String) {
    let seed = input.unwrap_or("bgvyzdsv");
    let accept5 = |d: &Digest| d.starts_with(&[0; 2]) && d[2] <= 15;
    let accept6 = |d: &Digest| d.starts_with(&[0; 3]);

    let mut first = 0;
    let mut second = 0;

    for i in 1.. {
        let input = format!("{}{}", seed, i);
        let digest = md5::compute(input);
        if first == 0 && accept5(&digest) {
            first = i;
        } else if accept6(&digest) {
            second = i;
            break;
        }
    }

    (first.to_string(), second.to_string())
}
