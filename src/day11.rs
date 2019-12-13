use crate::intcode::Computer;
use aoc_runner_derive::{aoc, aoc_generator};
use std::cmp::max;
use std::cmp::min;
use std::collections::HashMap;
use std::num::ParseIntError;

#[aoc_generator(day11)]
fn one_line_many_numbers(input: &str) -> Result<Vec<i64>, ParseIntError> {
    input.split(',').map(str::parse).collect()
}

#[aoc(day11, part1)]
fn solver1(program: &[i64]) -> usize {
    let mut hm = HashMap::<(i64, i64), i64>::new();
    let (mut x, mut y, mut dir) = (0, 0, 0);
    let mut robot = Computer::from(program.to_vec());
    while let Some(new_color) = robot.run_until_output_with(|| *hm.get(&(x, y)).unwrap_or(&0)) {
        hm.insert((x, y), new_color);
        let turn = match robot.run_until_output_with(|| new_color) {
            Some(t) => t * 2 - 1,
            None => break,
        };
        dir = (dir + turn + 4) % 4;
        match dir {
            0 => y -= 1,
            1 => x += 1,
            2 => y += 1,
            _ => x -= 1,
        }
    }
    hm.len()
}

#[aoc(day11, part2)]
fn solver2(program: &[i64]) -> String {
    let mut hm = HashMap::<(i64, i64), i64>::new();
    hm.insert((0, 0), 1);
    let (mut x, mut y, mut xmin, mut ymin, mut xmax, mut ymax, mut dir) = (0, 0, 0, 0, 0, 0, 0);
    let mut robot = Computer::from(program.to_vec());
    while let Some(new_color) = robot.run_until_output_with(|| *hm.get(&(x, y)).unwrap_or(&0)) {
        xmin = min(x, xmin);
        xmax = max(x, xmax);
        ymin = min(y, ymin);
        ymax = max(y, ymax);
        hm.insert((x, y), new_color);
        let turn = match robot.run_until_output_with(|| new_color) {
            Some(t) => t * 2 - 1,
            None => break,
        };
        dir = (dir + turn + 4) % 4;
        match dir {
            0 => y -= 1,
            1 => x += 1,
            2 => y += 1,
            _ => x -= 1,
        }
    }

    let mut result = String::new();
    result += "\n";

    for y in ymin..=ymax {
        for x in xmin..=xmax {
            result += match *hm.get(&(x, y)).unwrap_or(&0) {
                0 => ".",
                _ => "#",
            }
        }
        result += "\n";
    }
    result
}

#[cfg(test)]
mod tests {
    // No tests in the puzzle text :(
}
