use crate::intcode::Computer;
use aoc_runner_derive::{aoc, aoc_generator};
use std::num::ParseIntError;

#[aoc_generator(day5)]
fn one_line_many_numbers(input: &str) -> Result<Vec<isize>, ParseIntError> {
    input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|n| n.parse())
        .collect()
}

#[aoc(day5, part1)]
fn solver1(input: &[isize]) -> isize {
    *Computer::new(input.to_vec(), vec![1])
        .run()
        .last()
        .expect("No output produced")
}

#[aoc(day5, part2)]
fn solver2(input: &[isize]) -> isize {
    *Computer::new(input.to_vec(), vec![5])
        .run()
        .last()
        .expect("No output produced")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run1() {
        assert_eq!(
            Computer::from(vec![1002, 4, 3, 4, 33]).run(),
            &Vec::<isize>::new()
        );
        assert_eq!(Computer::new(vec![3, 0, 4, 0, 99], vec![37]).run()[0], 37);
        assert_eq!(solver1(&[3, 0, 4, 0, 99]), 1);
    }

    #[test]
    fn test_run2() {
        assert_eq!(
            *Computer::new(vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8], vec![8])
                .run()
                .last()
                .unwrap(),
            1
        );
        assert_eq!(
            *Computer::new(vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8], vec![7])
                .run()
                .last()
                .unwrap(),
            0
        );
        assert_eq!(
            *Computer::new(vec![3, 3, 1108, -1, 8, 3, 4, 3, 99], vec![8])
                .run()
                .last()
                .unwrap(),
            1
        );
        assert_eq!(
            *Computer::new(
                vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9],
                vec![15]
            )
            .run()
            .last()
            .unwrap(),
            1
        );
        assert_eq!(
            *Computer::new(
                vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9],
                vec![0]
            )
            .run()
            .last()
            .unwrap(),
            0
        );
        assert_eq!(
            *Computer::new(
                vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1],
                vec![15]
            )
            .run()
            .last()
            .unwrap(),
            1
        );
        assert_eq!(
            *Computer::new(
                vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1],
                vec![0]
            )
            .run()
            .last()
            .unwrap(),
            0
        );
        assert_eq!(
            *Computer::new(
                vec![
                    3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0,
                    36, 98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46,
                    1101, 1000, 1, 20, 4, 20, 1105, 1, 46, 98, 99
                ],
                vec![2]
            )
            .run()
            .last()
            .unwrap(),
            999
        );
        assert_eq!(
            *Computer::new(
                vec![
                    3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0,
                    36, 98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46,
                    1101, 1000, 1, 20, 4, 20, 1105, 1, 46, 98, 99
                ],
                vec![8]
            )
            .run()
            .last()
            .unwrap(),
            1000
        );
        assert_eq!(
            *Computer::new(
                vec![
                    3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0,
                    36, 98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46,
                    1101, 1000, 1, 20, 4, 20, 1105, 1, 46, 98, 99
                ],
                vec![13]
            )
            .run()
            .last()
            .unwrap(),
            1001
        );
    }
}
