use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    // Your solution here...
    let mut current_pos = 50;
    let mut result: u32 = 0;
    let rotations = read_to_string("input/1.txt").unwrap();
    for rotation in rotations.lines() {
        let value: String = rotation.parse().unwrap();
        let direction: char = value.chars().next().unwrap();

        let distance: u32 = value[1..].parse().unwrap();
        let full_rotations = distance / 100;
        result += full_rotations;
        let remaining = (distance % 100) as i32;

        match direction {
            'R' => {
                // Move right
                if current_pos + remaining >= 100 {
                    result += 1;
                }
                current_pos = (current_pos + remaining) % 100;
            }
            'L' => {
                // Move left
                if current_pos > 0 && current_pos - remaining <= 0 {
                    result += 1;
                }
                current_pos = (current_pos - remaining + 100) % 100;
            }
            _ => panic!("Invalid direction: {}", direction),
        }
        current_pos %= 100;
    }

    (Solution::U32(result), Solution::U32(0))
}
