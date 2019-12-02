use aoc_runner_derive::{aoc, aoc_generator};
use std::num::ParseIntError;

#[aoc_generator(day2)]
fn one_line_many_numbers(input: &str) -> Result<Vec<usize>, ParseIntError> {
    input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|n| n.parse())
        .collect()
}

struct Program {
    data: Vec<usize>,
    cursor: usize,
    halted: bool,
}

impl Program {
    fn one_step(&mut self) {
        match self.data[self.cursor] {
            1 => {
                let replacement_pos = self.data[self.cursor + 3];
                let (in1, in2) = (self.data[self.cursor + 1], self.data[self.cursor + 2]);
                self.data[replacement_pos] = self.data[in1] + self.data[in2];
                self.cursor += 4;
            }
            2 => {
                let replacement_pos = self.data[self.cursor + 3];
                let (in1, in2) = (self.data[self.cursor + 1], self.data[self.cursor + 2]);
                self.data[replacement_pos] = self.data[in1] * self.data[in2];
                self.cursor += 4;
            }
            99 => self.halted = true,
            n => panic!("Unknown opcode {}", n),
        }
    }

    fn run_with_input(&mut self, in1: usize, in2: usize) -> usize {
        self.data[1] = in1;
        self.data[2] = in2;
        self.run()
    }

    fn run(&mut self) -> usize {
        while !self.halted {
            self.one_step();
        }
        self.data[0]
    }
}

impl From<Vec<usize>> for Program {
    fn from(v: Vec<usize>) -> Self {
        Program {
            data: v,
            cursor: 0,
            halted: false,
        }
    }
}

#[aoc(day2, part1)]
fn solver1(input: &[usize]) -> usize {
    Program::from(input.to_vec()).run_with_input(12, 2)
}

#[aoc(day2, part2)]
fn solver2(input: &[usize]) -> Option<usize> {
    let desired_output = 19690720;
    for i in 0..100usize {
        for j in 0..100usize {
            if Program::from(input.to_vec()).run_with_input(i, j) == desired_output {
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
        assert_eq!(Program::from(vec![1usize, 0, 0, 0, 99]).run(), 2);
    }
}
