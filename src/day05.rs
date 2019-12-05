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

struct Computer {
    memory: Vec<isize>,
    procnt: isize,
    halted: bool,
    input: Vec<isize>,
    output: Vec<isize>,
}

impl Computer {
    fn new(program_data: Vec<isize>, input: Vec<isize>) -> Self {
        Self {
            memory: program_data,
            procnt: 0,
            halted: false,
            input: input,
            output: Vec::new(),
        }
    }
    fn one_step(&mut self) {
        let (mask, opcode) = self.instruction();
        match opcode {
            1 => {
                let replacement_pos = self.memory[self.procnt as usize + 3]; // never immediate mode
                let p = self.params(2, mask);
                self.memory[replacement_pos as usize] = p[0] + p[1];
                self.procnt += 4;
            }
            2 => {
                let replacement_pos = self.memory[self.procnt as usize + 3]; // never immediate mode
                let p = self.params(2, mask);
                self.memory[replacement_pos as usize] = p[0] * p[1];
                self.procnt += 4;
            }
            3 => {
                let replacement_pos = self.memory[self.procnt as usize + 1]; // never immediate mode
                self.memory[replacement_pos as usize] =
                    self.input.pop().expect("Input was taken but none is left");
                self.procnt += 2;
            }
            4 => {
                let p = self.params(1, mask);
                self.output.push(p[0]);
                self.procnt += 2;
            }
            5 => {
                let p = self.params(2, mask);
                if p[0] != 0 {
                    self.procnt = p[1];
                } else {
                    self.procnt += 3;
                }
            }
            6 => {
                let p = self.params(2, mask);
                if p[0] == 0 {
                    self.procnt = p[1];
                } else {
                    self.procnt += 3;
                }
            }
            7 => {
                let replacement_pos = self.memory[self.procnt as usize + 3]; // never immediate mode
                let p = self.params(2, mask);
                self.memory[replacement_pos as usize] = if p[0] < p[1] { 1 } else { 0 };
                self.procnt += 4;
            }
            8 => {
                let replacement_pos = self.memory[self.procnt as usize + 3]; // never immediate mode
                let p = self.params(2, mask);
                self.memory[replacement_pos as usize] = if p[0] == p[1] { 1 } else { 0 };
                self.procnt += 4;
            }
            99 => self.halted = true,
            n => panic!("Unknown opcode {}", n),
        }
    }

    fn run(&mut self) -> &Vec<isize> {
        while !self.halted {
            self.one_step();
        }
        &self.output
    }

    fn instruction(&self) -> (Mask, Opcode) {
        let opcode = self.memory[self.procnt as usize] % 100;
        let mut mask = Vec::new();
        let mut digits = self.memory[self.procnt as usize] / 100;
        while digits > 0 {
            mask.push(match digits % 10 {
                0 => Mode::Position,
                _ => Mode::Immediate,
            });
            digits /= 10;
        }
        (Mask(mask), opcode)
    }

    fn read(&self, operand: isize, mode: &Mode) -> isize {
        match &mode {
            Mode::Immediate => operand,
            Mode::Position => self.memory[operand as usize],
        }
    }

    fn params(&self, amount: usize, mask: Mask) -> Vec<isize> {
        (0..amount)
            .map(|i| self.read(self.memory[self.procnt as usize + i + 1], mask.get(i)))
            .collect()
    }
}

type Opcode = isize;
struct Mask(Vec<Mode>);

#[derive(Debug, Copy, Clone)]
enum Mode {
    Immediate,
    Position,
}

impl Mask {
    fn get(&self, idx: usize) -> &Mode {
        self.0.get(idx).unwrap_or(&Mode::Position)
    }
}

impl From<Vec<isize>> for Computer {
    fn from(v: Vec<isize>) -> Self {
        Computer::new(v, Vec::new())
    }
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
