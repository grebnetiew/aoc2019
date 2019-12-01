use aoc_runner_derive::{aoc, aoc_generator};
use std::num::ParseIntError;

#[aoc_generator(day1)]
fn number_per_line(input: &str) -> Result<Vec<u32>, ParseIntError> {
    input.lines().map(|l| l.trim().parse()).collect()
}

#[aoc(day1, part1)]
fn solver1(input: &[u32]) -> u32 {
    input.iter().map(|mass| mass / 3 - 2).sum()
}

#[aoc(day1, part2)]
fn solver2(input: &[u32]) -> u32 {
    input
        .iter()
        .map(|&mass| {
            let mut mass = mass;
            let mut totalfuel = 0u32;
            while let Some(new_mass) = (mass / 3).checked_sub(2) {
                mass = new_mass;
                totalfuel += mass;
            }
            totalfuel
        })
        .sum()
}

struct Fueler(u32);
impl Iterator for Fueler {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.0 = (self.0 / 3).checked_sub(2)?;
        Some(self.0)
    }
}

#[aoc(day1, part2, Iterator)]
fn solver2a(input: &[u32]) -> u32 {
    input.iter().map(|&mass| Fueler(mass).sum::<u32>()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(solver1(&[12]), 2);
        assert_eq!(solver1(&[14]), 2);
        assert_eq!(solver1(&[1969]), 654);
        assert_eq!(solver1(&[100756]), 33583);
    }

    #[test]
    fn part2_example() {
        assert_eq!(solver2(&[14]), 2);
        assert_eq!(solver2(&[1969]), 966);
        assert_eq!(solver2(&[100756]), 50346);
        assert_eq!(solver2a(&[14]), 2);
        assert_eq!(solver2a(&[1969]), 966);
        assert_eq!(solver2a(&[100756]), 50346);
    }
}
