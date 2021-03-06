use crate::intcode::Computer;
use aoc_runner_derive::{aoc, aoc_generator};
use std::num::ParseIntError;

#[aoc_generator(day2)]
fn one_line_many_numbers(input: &str) -> Result<Vec<i64>, ParseIntError> {
    input.split(',').map(str::parse).collect()
}

fn compute(input: &[i64], i: i64, j: i64) -> i64 {
    let mut modified_input = input.to_owned();
    modified_input[1] = i;
    modified_input[2] = j;
    let mut computer = Computer::from(modified_input);
    computer.run();
    computer.mem_first()
}

#[aoc(day2, part1)]
fn solver1(input: &[i64]) -> i64 {
    compute(input, 12, 2)
}

#[aoc(day2, part2)]
fn solver2(input: &[i64]) -> Option<i64> {
    let desired_output = 19_690_720;
    for i in 0..100 {
        for j in 0..100 {
            if compute(input, i, j) == desired_output {
                return Some(i * 100 + j);
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        assert_eq!(compute(&[1, 0, 0, 0, 99], 0, 0), 2);
    }
}
