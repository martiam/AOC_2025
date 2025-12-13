use crate::{Solution, SolutionPair};
use std::cmp::{Reverse, max};
use std::fs::read_to_string;
use std::ops::RangeInclusive;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/5.txt").unwrap();
    let parts: Vec<&str> = input.split("\n\n").collect();
    let (ranges_input, ids_input) = match parts.as_slice() {
        &[r, i] => (r, i),
        _ => panic!("Expected exactly 2 sections separated by a blank line"),
    }; 

    let ranges: Vec<RangeInclusive<u128>> = ranges_input
        .lines()
        .map(|line| {
            let range_parts: Vec<&str> = line.split("-").collect();
            let (start, end) = match range_parts.as_slice() {
                &[s, e] => (s, e),
                _ => panic!("Expected a range compatible format"),
            };
            start.parse::<u128>().unwrap()..=end.parse::<u128>().unwrap()
        })
        .collect();

    let ids: Vec<u128> = ids_input
        .lines()
        .map(|line| line.parse::<u128>().unwrap())
        .collect();

    let mut sol1: u64 = 0;

    for id in &ids {
        for range in &ranges {
            if range.contains(id) {
                sol1 += 1;
                break
            }
        }
    }

    // Part 2
    let mut sol2: u64 = 0;
    let mut ranges2 = ranges.clone();
    ranges2.sort_by_key(|r| (*r.start(), Reverse(*r.end())));
    let (first, rest) = ranges2
        .split_first()
        .expect("Empty ranges");
    let mut current_start = *first.start();
    let mut current_end = *first.end();

    for range in rest {
        if current_end >= range.start() - 1 {
            current_end = max(current_end, *range.end());
        } else {
            sol2 += (current_end - current_start + 1) as u64;
            current_start = *range.start();
            current_end = *range.end();
        }
    }
    sol2 += (current_end - current_start + 1) as u64;

    (Solution::from(sol1), Solution::from(sol2))
}
