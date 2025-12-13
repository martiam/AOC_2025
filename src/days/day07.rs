use crate::{etc::Grid, Solution, SolutionPair};
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/7.txt").unwrap();
    let grid = Grid::from_str(&input);
    let mut active_beams: HashSet<usize> = HashSet::new();
    let mut sol1: u64 = 0; // split count

    for i in 0..grid.width() {
        if grid[(i, 0)] == 'S' {
            active_beams.insert(i);
        }
    }

    for row in 1..grid.height() {
        let mut next_beams: HashSet<usize> = HashSet::new();
        for &col in &active_beams {
            if grid[(col, row)] == '^' {
                sol1 += 1;
                if col > 0 {
                    next_beams.insert(col - 1);
                }
                if col + 1 < grid.width() {
                    next_beams.insert(col + 1);
                }
            } else {
                next_beams.insert(col);
            }
        }
        active_beams = next_beams;
    }

    // Part 2
    let mut active_beams: HashMap<usize, u64> = HashMap::new();

    for i in 0..grid.width() {
        if grid[(i, 0)] == 'S' {
            active_beams.insert(i, 1);
        }
    }

    for row in 1..grid.height() {
        let mut next_beams: HashMap<usize, u64> = HashMap::new();
        for (col, count) in &active_beams {
            if grid[(*col, row)] == '^' {
                if *col > 0 {
                    *next_beams.entry(*col - 1).or_insert(0) += count;
                }
                if *col + 1 < grid.width() {
                    *next_beams.entry(*col + 1).or_insert(0) += count;
                }
            } else {
                *next_beams.entry(*col).or_insert(0) += count
            }
        }
        active_beams = next_beams;
    }
    let sol2: u64 = active_beams.values().sum();

    (Solution::from(sol1), Solution::from(sol2))
}
