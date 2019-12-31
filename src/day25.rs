extern crate text_io;

use crate::intcode::Computer;
use aoc_runner_derive::{aoc, aoc_generator};
use std::fmt;
use std::num::ParseIntError;

#[aoc_generator(day25)]
fn one_line_many_numbers(input: &str) -> Result<Vec<i64>, ParseIntError> {
    input.split(',').map(str::parse).collect()
}

#[derive(Debug)]
enum Never {}

impl fmt::Display for Never {
    fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
        Ok(())
    }
}

#[aoc(day25, part1)]
fn interactive(program: &[i64]) -> Option<Never> {
    let mut computer = Computer::from(program.to_vec());
    let mut buffer = String::new();
    loop {
        let reader = || {
            if buffer.is_empty() {
                buffer = read!("{}\n");
                buffer += "\n";
            }
            buffer.remove(0) as u8 as i64
        };
        print!("{}", computer.run_until_output_with(reader)? as u8 as char)
    }
}

#[cfg(test)]
mod tests {
    // No tests in the puzzle text :(
}
