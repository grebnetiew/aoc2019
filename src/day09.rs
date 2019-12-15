use crate::intcode::Computer;
use aoc_runner_derive::{aoc, aoc_generator};
use std::num::ParseIntError;

#[aoc_generator(day9)]
fn one_line_many_numbers(input: &str) -> Result<Vec<i64>, ParseIntError> {
    input.split(',').map(str::parse).collect()
}

#[aoc(day9, part1)]
fn solver1(program: &[i64]) -> Option<i64> {
    Computer::from(program.to_vec()).run_until_output_with(|| 1)
}

#[aoc(day9, part2)]
fn solver2(program: &[i64]) -> Option<i64> {
    Computer::from(program.to_vec()).run_until_output_with(|| 2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_large_numbers() {
        assert_eq!(
            solver1(&[104, 1_125_899_906_842_624, 99]),
            Some(1_125_899_906_842_624)
        );
    }
}
