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
    fn new(programData: Vec<isize>, input: Vec<isize>) -> Self {
        Self {
            memory: programData,
            procnt: 0,
            halted: false,
            input: input,
            output: Vec::new(),
        }
    }
    fn one_step(&mut self) {
        match self.memory[self.procnt] {
            1 => {
                let replacement_pos = self.memory[self.procnt + 3];
                let (in1, in2) = (self.memory[self.procnt + 1], self.memory[self.procnt + 2]);
                self.memory[replacement_pos] = self.memory[in1] + self.memory[in2];
                self.procnt += 4;
            }
            2 => {
                let replacement_pos = self.memory[self.procnt + 3];
                let (in1, in2) = (self.memory[self.procnt + 1], self.memory[self.procnt + 2]);
                self.memory[replacement_pos] = self.memory[in1] * self.memory[in2];
                self.procnt += 4;
            }
            99 => self.halted = true,
            n => panic!("Unknown opcode {}", n),
        }
    }

    fn run(&mut self) -> isize {
        while !self.halted {
            self.one_step();
        }
        self.memory[0]
    }

    fn instruction(ins: isize) -> (Mask, Opcode) {
        let opcode = ins % 100;
        
    }

    type Opcode = isize;
    type Mask = Vec<Mode>;
    enum Mode {
        Direct,
        Reference,
    }
}

impl From<Vec<isize>> for Computer {
    fn from(v: Vec<isize>) -> Self {
        Computer::new(v, Vec::new());
    }
}

#[aoc(day5, part1)]
fn solver1(input: &[isize]) -> isize {
    Computer::new(input.to_vec(), vec![1])
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        assert_eq!(Computer::from(vec![1usize, 0, 0, 0, 99]).run(), 2);
    }
}
