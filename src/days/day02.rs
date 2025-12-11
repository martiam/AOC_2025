use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

fn find_invalid_ids_part_1(range: &(u128, u128)) -> Vec<u128> {
    let (start, end) = *range;
    let mut invalid_ids: Vec<u128> = vec![];
    for id in start..=end {
        // Since the young Elf was just doing silly patterns, you can find the invalid IDs by looking for any ID which is made only of some sequence of digits repeated twice. So, 55 (5 twice), 6464 (64 twice), and 123123 (123 twice) would all be invalid IDs.
        let id_str = id.to_string();
        let len = id_str.len();
        if len % 2 != 0 {
            continue
        }
        let mid = len / 2;
        if &id_str[0..mid] == &id_str[mid..len] {
            invalid_ids.push(id);
        }
    }
    invalid_ids
}

fn find_invalid_ids_part_2(range: &(u128, u128)) -> Vec<u128> {
    let (start, end) = *range;
    let mut invalid_ids: Vec<u128> = vec![];
    for id in start..=end {
        // Now, an ID is invalid if it is made only of some sequence of digits repeated at least twice. So, 12341234 (1234 two times), 123123123 (123 three times), 1212121212 (12 five times), and 1111111 (1 seven times) are all invalid IDs.
        let id_str = id.to_string();
        let len = id_str.len();
        for sub_len in 1..=(len / 2) {
            if len % sub_len != 0 {
                continue;
            }
            let sub_str = &id_str[0..sub_len];
            let mut repeated = String::new();
            for _ in 0..(len / sub_len) {
                repeated.push_str(sub_str);
            }
            if repeated == id_str {
                invalid_ids.push(id);
                break;
            }
        }
    }
    invalid_ids
}

pub fn solve() -> SolutionPair {
    let mut ranges: Vec<(u128, u128)> = vec![];
    let input = read_to_string("input/2.txt").unwrap();
    let mut invalid_ids: Vec<u128> = vec![];
    // only a single line of input
    for part in input.trim().split(',') {
        let bounds: Vec<u128> = part
            .trim()
            .split('-')
            .map(|x| x.parse().unwrap())
            .collect();
        ranges.push((bounds[0], bounds[1]));
    }

    for (start, end) in ranges.iter() {
        let mut ids = find_invalid_ids_part_1(&(*start, *end));
        invalid_ids.append(&mut ids);
    }

    // add all invalid ids together for solution 1
    let sum_invalid: u128 = invalid_ids.iter().sum();
    let sol1: u64 = sum_invalid as u64;

    invalid_ids.clear();
    for (start, end) in ranges.iter() {
        let mut ids = find_invalid_ids_part_2(&(*start, *end));
        invalid_ids.append(&mut ids);
    }
    let sol2: u128 = invalid_ids.iter().sum();

    (Solution::from(sol1), Solution::from(sol2))
}
