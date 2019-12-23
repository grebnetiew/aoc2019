use crate::intcode::Computer;
use aoc_runner_derive::{aoc, aoc_generator};
use std::num::ParseIntError;

#[aoc_generator(day21)]
fn one_line_many_numbers(input: &str) -> Result<Vec<i64>, ParseIntError> {
    input.split(',').map(str::parse).collect()
}

#[aoc(day21, part1)]
fn solver1(program: &[i64]) -> i64 {
    // Idea: You will land on D if you jump, so if D is ground (true)
    // and any other tile is a hole (false), jump.
    let springcode = "NOT A T\n\
                      NOT B J\n\
                      OR T J\n\
                      NOT C T\n\
                      OR T J\n\
                      AND D J\n\
                      WALK\n";
    let mut computer = Computer::new(
        program.to_vec(),
        springcode.chars().rev().map(|c| c as u8 as i64).collect(),
    );
    let output = computer.run();
    output[output.len() - 1]
}

#[aoc(day21, part2)]
fn solver2(program: &[i64]) -> i64 {
    // Now, we need to be lazy with our jumps if possible.
    // Only jump if from the landing spot, you can safely continue.
    // If D is ground, and (H is ground, or (I and E are ground), or
    // E and F are ground), jump.
    let springcode = "OR E J\n\
                      AND F J\n\
                      OR E T\n\
                      AND I T\n\
                      OR T J\n\
                      OR H J\n\
                      AND D J\n\
                      NOT A T\n\
                      NOT T T\n\
                      AND B T\n\
                      AND C T\n\
                      NOT T T\n\
                      AND T J\n\
                      RUN\n";
    let mut computer = Computer::new(
        program.to_vec(),
        springcode.chars().rev().map(|c| c as u8 as i64).collect(),
    );
    let output = computer.run();
    output[output.len() - 1]
}

#[cfg(test)]
mod tests {
    // No tests in the puzzle text :(
}
