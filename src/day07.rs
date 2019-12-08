extern crate permutohedron;

use crate::intcode::Computer;
use aoc_runner_derive::{aoc, aoc_generator};
use permutohedron::heap_recursive;
use std::num::ParseIntError;

#[aoc_generator(day7)]
fn one_line_many_numbers(input: &str) -> Result<Vec<isize>, ParseIntError> {
    input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|n| n.parse())
        .collect()
}

#[aoc(day7, part1)]
fn solver1(program: &[isize]) -> isize {
    let mut max_thrust = 0;
    let mut possible_settings = [0, 1, 2, 3, 4];

    // heap_recursive is an algorithm that produces permutations of the data
    heap_recursive(&mut possible_settings, |permutation| {
        let mut last_out = 0;
        // Chain the amplifiers to each other
        for &setting in permutation.iter() {
            last_out = Computer::new(program.to_vec(), vec![last_out, setting])
                .run_until_output()
                .unwrap();
        }
        // Collect maximum thrust
        if last_out > max_thrust {
            max_thrust = last_out;
        }
    });
    max_thrust
}

#[aoc(day7, part2)]
fn solver2(program: &[isize]) -> isize {
    let mut max_thrust = 0;
    let mut possible_settings = [5, 6, 7, 8, 9];

    heap_recursive(&mut possible_settings, |permutation| {
        // Set up five amplifiers for the feedback loop
        let mut amps = Vec::with_capacity(5);
        for &setting in permutation.iter() {
            amps.push(Computer::new(program.to_vec(), vec![setting]));
        }

        // Keep running them in a chain until one of them halts
        let mut last_out = 0;
        'outer: loop {
            for amp in amps.iter_mut() {
                amp.more_input(last_out);
                match amp.run_until_output() {
                    Some(output) => last_out = output,
                    None => break 'outer,
                }
            }
            if last_out > max_thrust {
                max_thrust = last_out;
            }
        }
    });
    max_thrust
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run1() {
        assert_eq!(
            solver1(&[3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0]),
            43210
        );
        assert_eq!(
            solver1(&[
                3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4,
                23, 99, 0, 0
            ]),
            54321
        );
        assert_eq!(
            solver1(&[
                3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33,
                1, 33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0
            ]),
            65210
        );
    }

    #[test]
    fn test_run2() {
        assert_eq!(
            solver2(&[
                3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28,
                -1, 28, 1005, 28, 6, 99, 0, 0, 5
            ]),
            139_629_729
        );
        assert_eq!(
            solver2(&[
                3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001,
                54, -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53,
                55, 53, 4, 53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10
            ]),
            18216
        );
    }

}
