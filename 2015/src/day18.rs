// https://adventofcode.com/2015/day/18

use self::grid::Grid;
use std::fs;

pub fn day18(filename: Option<&str>) -> (String, String) {
    let input = fs::read_to_string(filename.unwrap_or("input/18")).unwrap();
    let mut a = Grid::new(&input, 100);
    let mut b = Grid::with_corners_on(&input, 100);

    a.advance(100);
    b.advance(100);

    (a.num_lights_on().to_string(), b.num_lights_on().to_string())
}

// A separate module to keep private methods... private.
mod grid {
    use std::collections::HashSet;

    pub struct Grid {
        side: usize,
        grid: Vec<u32>,
        force_on: HashSet<usize>,
    }

    impl Grid {
        // For part 1.
        pub fn new(input: &str, side: usize) -> Grid {
            // We will use a (side+2)×(side+2) grid to avoid dealing with
            // special “neighbour formulas” for cells in the borders.
            let side = side + 2;
            let mut grid = vec![0; side + 1];

            for c in input.chars() {
                match c {
                    '#' => grid.push(1),
                    '.' => grid.push(0),
                    '\n' => {
                        grid.push(0);
                        grid.push(0);
                        assert_eq!((grid.len() - 1) % side, 0);
                    }
                    _ => panic!("unexpected character in grid: {:?}", c),
                }
            }
            grid.append(&mut vec![0; side - 1]);
            assert_eq!(grid.len(), side * side);
            Grid {
                grid,
                side,
                force_on: HashSet::new(),
            }
        }

        // For part 2.
        pub fn with_corners_on(input: &str, side: usize) -> Grid {
            let mut grid = Grid::new(input, side);
            grid.force_on.insert(grid.side + 1);
            grid.force_on.insert(grid.side * 2 - 2);
            grid.force_on.insert(grid.side * (grid.side - 2) + 1);
            grid.force_on.insert(grid.side * (grid.side - 1) - 2);
            for idx in grid.force_on.iter() {
                grid.grid[*idx] = 1;
            }
            grid
        }

        pub fn num_lights_on(&self) -> u32 {
            self.grid.iter().sum()
        }

        pub fn advance(&mut self, steps: u32) {
            let mut altgrid = self.grid.clone();

            for _ in 0..steps {
                self.advance_step(&mut altgrid);
                self.grid.swap_with_slice(&mut altgrid);
            }
        }

        fn advance_step(&self, dst: &mut [u32]) {
            let side = self.side;
            assert_eq!(dst.len(), self.grid.len());

            for i in 0..self.grid.len() {
                if i < side || i % side == 0 || (i + 1) % side == 0 || i >= side * (side - 1) {
                    continue;
                }
                if self.force_on.contains(&i) {
                    continue;
                }
                dst[i] = self.next_status(i);
            }
        }

        fn next_status(&self, idx: usize) -> u32 {
            let side = self.side;
            let mut neighbours_on = self.grid[idx - side] + self.grid[idx + side];

            let up = idx - 1;
            neighbours_on += self.grid[up] + self.grid[up - side] + self.grid[up + side];

            let down = idx + 1;
            neighbours_on += self.grid[down] + self.grid[down - side] + self.grid[down + side];

            match (self.grid[idx], neighbours_on) {
                (1, 2) => 1,
                (1, 3) => 1,
                (1, _) => 0,
                (0, 3) => 1,
                (x, _) => x,
            }
        }
    }
}
