use crate::intcode::Computer;
use aoc_runner_derive::{aoc, aoc_generator};
use std::num::ParseIntError;

#[aoc_generator(day9)]
fn one_line_many_numbers(input: &str) -> Result<Vec<isize>, ParseIntError> {
    input.split(',').map(str::parse).collect()
}

#[aoc(day9, part1)]
fn solver1(program: &[isize]) -> isize {
    Computer::new(program.to_vec(), vec![1])
        .run_until_output()
        .unwrap()
}

#[aoc(day9, part2)]
fn solver2(program: &[isize]) -> isize {
    Computer::new(program.to_vec(), vec![2])
        .run_until_output()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run1() {
        assert_eq!(
            solver1(&[104, 1_125_899_906_842_624, 99]),
            1_125_899_906_842_624
        );
    }

    #[test]
    fn test_run2() {}

}
