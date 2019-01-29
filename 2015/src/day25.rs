// https://adventofcode.com/2015/day/25

#[rustfmt::skip]
const CODES: [[u64; 6]; 6] = [
    [20151125, 18749137, 17289845, 30943339, 10071777, 33511524],
    [31916031, 21629792, 16929656,  7726640, 15514188,  4041754],
    [16080970,  8057251,  1601130,  7981243, 11661866, 16474243],
    [24592653, 32451966, 21345942,  9380097, 10600672, 31527494],
    [   77061, 17552253, 28094349,  6899651,  9250759, 31663883],
    [33071741,  6796745, 25397450, 24659492,  1534922, 27995004],
];

pub fn day25(filename: Option<&str>) -> (String, String) {
    let input = crate::readline(filename.unwrap_or("input/25"));
    let row: usize;
    let col: usize;
    scan!(input.bytes() => "To continue, please consult the code grid in the manual.  \
                            Enter the code at row {}, column {}.", row, col);

    let next = |prev| (prev * 252533) % 33554393;
    let cantor = Cantor::new();
    let mut prev = 0;

    for (r, c) in cantor {
        let (i, j) = (r - 1, c - 1);
        if i < CODES.len() && j < CODES[i].len() {
            if prev > 0 {
                assert_eq!(next(prev), CODES[i][j]);
            }
            prev = CODES[i][j];
        } else {
            prev = next(prev);
        }
        if r == row && c == col {
            break;
        }
    }

    (prev.to_string(), "".to_owned())
}

struct Cantor {
    row: usize,
    col: usize,
}

impl Cantor {
    fn new() -> Cantor {
        // (2, 0) produces (1, 1) as the first value of the iterator.
        Cantor { row: 2, col: 0 }
    }
}

impl Iterator for Cantor {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.row == 1 {
            self.row = self.col + 1;
            self.col = 1;
        } else {
            self.row -= 1;
            self.col += 1;
        }
        Some((self.row, self.col))
    }
}
