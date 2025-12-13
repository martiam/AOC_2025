use crate::{
    etc::{Grid, Point},
    Solution, SolutionPair,
};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

fn count_cell_neighbors(grid: &Grid<char>, pos: Point) -> u8 {
    let mut count: u8 = 0;
    let offsets = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        // (0, 0), // skip the center cell
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    for (dx, dy) in offsets {
        let neighbor_pos = Point::new(pos.x + dx, pos.y + dy);
        if let Some(&ch) = grid.get(neighbor_pos) {
            if ch == '@' { count += 1; }
        }
    }
    count
}

fn find_all_accessible_rolls(grid: &Grid<char>) -> Vec<Point> {
    let mut result = Vec::new();
    for (pos, &cell) in grid.enumerate() {
        if cell == '@' {
            let neighbor_count = count_cell_neighbors(&grid, pos);
            if neighbor_count < 4 {
                result.push(pos)
            }
        }
    }

    result
}

fn remove_all(accessible: &Vec<Point>, grid: &mut Grid<char>) {
    for pos in accessible {
        grid[*pos] = '.';
    }
}

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/4.txt").expect("Error parsing input");

    let grid = Grid::from_str(&input);

    let mut sol1: u64 = 0;

    for (pos, &cell) in grid.enumerate() {
        if cell == '@' {
            let neighbor_count = count_cell_neighbors(&grid, pos);
            if neighbor_count < 4 {
                sol1 += 1;
            }
        }
    }

    let mut sol2: u64 = 0;
    let mut grid2 = grid.clone();
    loop {
        let accessible = find_all_accessible_rolls(&grid2);
        if accessible.is_empty() {
            break
        }
        remove_all(&accessible, &mut grid2);
        sol2 += accessible.len() as u64;
    }


    (Solution::from(sol1), Solution::from(sol2))
}
