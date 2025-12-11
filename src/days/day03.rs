use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

fn find_largest_num_and_index(line: &str) -> (char, usize) {
    let mut largest_value = 0;
    let mut ch = '0';
    let mut largest_index = 0;
    for (i, c) in line.chars().enumerate() {
        let n = c.to_digit(10).unwrap();
        if n > largest_value {
            ch = c;
            largest_value = n;
            largest_index = i;
        }
    }
    (ch, largest_index)
}

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/3.txt").unwrap();
    let mut joltages: Vec<u128> = Vec::new();
    for line in input.lines() {
        let len = line.len();
        // find the largest number in line and its index
        let mut first: char;
        let first_index: usize;
        (first, first_index) = find_largest_num_and_index(line);
        let second: char;
        if first_index < len - 1 {
            let (c, _) = find_largest_num_and_index(&line[first_index + 1..]);
            second = c;
        } else {
            second = first;
            let (c, _) = find_largest_num_and_index(&line[..first_index]);
            first = c;
        }
        let joltage: u128 = format!("{}{}", first, second).parse().unwrap();
        // println!("Line: {}, First: {}, Second: {}, Joltage: {}", line, first, second, joltage);
        joltages.push(joltage);
    }
    let sol1: u128 = joltages.iter().sum();

    joltages.clear();

    for line in input.lines() {
        let len = line.len();
        let mut removes_left = len - 12;
        let mut line_clone = line;
        let mut final_string: String = String::new();

        while final_string.len() < 12 && removes_left < line_clone.len() {
            let (_, index) = find_largest_num_and_index(&line_clone[0..removes_left + 1]);
            final_string.push_str(&line_clone.chars().nth(index).unwrap().to_string());
            line_clone = &line_clone[index + 1..];
            removes_left -= index;
        }

        if removes_left != line_clone.len() {
            final_string.push_str(line_clone);
        }

        let joltage: u128 = final_string.parse().unwrap();
        joltages.push(joltage);
    }
    let sol2: u128 = joltages.iter().sum();

    (Solution::from(sol1), Solution::from(sol2))
}
