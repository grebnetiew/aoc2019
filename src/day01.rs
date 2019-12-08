use aoc_runner_derive::{aoc, aoc_generator};
use std::num::ParseIntError;

// An AOC Generator turns the input - read into a string - into something meaningful.
// In this case, there is one number per line, so we want a vector of numbers (or,
// if one of the lines contains garbage, an error).
#[aoc_generator(day1)]
fn number_per_line(input: &str) -> Result<Vec<u32>, ParseIntError> {
    input.lines().map(|l| l.trim().parse()).collect()
}

// A solver solves one of the two puzzles.
// This one solves Part 1.
// (Get a list of parts' mass; each part needs floor(m / 3 - 2) fuel, compute
// how much fuel is needed.)
#[aoc(day1, part1)]
fn solver1(input: &[u32]) -> u32 {
    input.iter().map(|mass| mass / 3 - 2).sum()
}

// A naive solution for part 2, which uses checked overflow to abort
// if a subtraction runs below 0.
// (Same as Part 1, but you need fuel for the fuel (per part) as well.)
#[aoc(day1, part2, Loop)]
fn solver2(input: &[u32]) -> u32 {
    input.iter().map(|&mass| totalmass(mass)).sum()
}

fn totalmass(mut mass: u32) -> u32 {
    let mut totalfuel = 0u32;
    while let Some(new_mass) = (mass / 3).checked_sub(2) {
        mass = new_mass;
        totalfuel += mass;
    }
    totalfuel
}

// While not inelegant, I didn't think the loop-based solution was very idiomatic.
// I decided to experiment with some more readable/neat/galaxybrained versions.
#[aoc(day1, part2, Recursion)]
fn solver2a(input: &[u32]) -> u32 {
    input.iter().map(|&mass| recursivemass(mass)).sum()
}

fn recursivemass(part: u32) -> u32 {
    match (part / 3).saturating_sub(2) {
        0 => 0,
        fuel => fuel + recursivemass(fuel),
    }
}

// Recursion is sometimes considered inefficient, so I tried to implement an
// iterator which gives the successive additional fuel requirements per part.
// So first the fuel for the part, then the fuel for the previous amount of
// fuel, and so on. It stops once it reaches 0 fuel.
struct Fueler(u32);
impl Iterator for Fueler {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.0 = (self.0 / 3).checked_sub(2)?;
        Some(self.0)
    }
}

// That way we can just turn each part's mass into a Fueler iterator, and sum
// all the fuel values it spits out. Neat!
#[aoc(day1, part2, Iterator)]
fn solver2b(input: &[u32]) -> u32 {
    input.iter().map(|&mass| Fueler(mass).sum::<u32>()).sum()
}

// For the tests, I just used the examples given in the puzzle text.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(solver1(&[12]), 2);
        assert_eq!(solver1(&[14]), 2);
        assert_eq!(solver1(&[1969]), 654);
        assert_eq!(solver1(&[100_756]), 33583);
    }

    #[test]
    fn part2_example() {
        assert_eq!(solver2(&[14]), 2);
        assert_eq!(solver2(&[1969]), 966);
        assert_eq!(solver2(&[100_756]), 50346);
        assert_eq!(solver2a(&[14]), 2);
        assert_eq!(solver2a(&[1969]), 966);
        assert_eq!(solver2a(&[100_756]), 50346);
        assert_eq!(solver2b(&[14]), 2);
        assert_eq!(solver2b(&[1969]), 966);
        assert_eq!(solver2b(&[100_756]), 50346);
    }
}
