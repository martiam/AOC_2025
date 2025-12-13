use crate::{Solution, SolutionPair};
use core::panic;
use std::{fs::read_to_string};

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/6.txt").unwrap();
    let lines: Vec<&str> = input.lines().collect();
    let rows: Vec<Vec<&str>> = lines.iter()
        .map(|line| line.split_whitespace().collect())
        .collect();
    let num_problems = rows.last().unwrap().len();
    
    let mut sol1: u64 = 0;

    // transpose
    for i in 0..num_problems {
        let column: Vec<&str> = rows.iter()
            .map(|row| row[i])
            .collect();
        let (op_str, nums_str) = column.split_last().unwrap();
        let operator = op_str.chars().next().unwrap();
        let numbers: Vec<u64> = nums_str.iter()
            .map(|s| s.parse::<u64>().unwrap())
            .collect();

        let result: u64 = match operator {
            '*' => numbers.iter().product(),
            '+' => numbers.iter().sum(),
            _ => panic!(),
        };

        sol1 += result;
    }

    // Part 2
    let mut sol2: u64 = 0;
    let lines: Vec<&str> = input.lines().collect();

    let char_grid: Vec<Vec<char>> = lines.iter()
        .map(|line| line.chars().collect())
        .collect();

    let max_width = char_grid.iter()
        .map(|row| row.len())
        .max()
        .unwrap();

    let mut current_numbers: Vec<u64> = Vec::new();
    let mut current_operator: char = ' ';

    for col_idx in 0..max_width {
        let column: Vec<char> = char_grid.iter()
            .map(|row| row
                .get(col_idx)
                .copied()
                .unwrap_or(' '))
            .collect();

        let (last_ch, digits) = column.split_last().unwrap();

        if *last_ch != ' ' {
            current_operator = *last_ch;
        }
        
        let number_str: String = digits.iter()
            .filter(|&&ch| ch != ' ')
            .collect();

        if number_str.is_empty() {
            for number in &current_numbers {
                println!("{number}");
            }
            println!("{current_operator}");
            match current_operator {
                '+' => { sol2 += current_numbers.iter().sum::<u64>(); }
                '*' => { sol2 += current_numbers.iter().product::<u64>(); },
                _ => panic!("Invalid operator"),
            };
            current_numbers.clear();
        } else {
            let number: u64 = number_str.parse().unwrap();
            current_numbers.push(number);
        }
    }

    match current_operator {
        '+' => { sol2 += current_numbers.iter().sum::<u64>(); }
        '*' => { sol2 += current_numbers.iter().product::<u64>(); },
        _ => panic!("Invalid operator"),
    };

    (Solution::from(sol1), Solution::from(sol2))
}
