use aoc_runner_derive::{aoc, aoc_generator};
use std::iter;
use std::num::ParseIntError;

#[aoc_generator(day16)]
fn digits(input: &str) -> Result<Vec<i32>, ParseIntError> {
    input.chars().map(|c| c.to_string().parse()).collect()
}

#[aoc(day16, part1)]
fn solver1(digits: &[i32]) -> String {
    let mut digits = digits.to_vec();
    for _ in 0..100 {
        digits = fft(&digits);
    }
    digits
        .iter()
        .take(8)
        .map(i32::to_string)
        .fold(String::from(""), |a, b| a + &b)
}

fn fft(digits: &[i32]) -> Vec<i32> {
    (0..digits.len())
        .map(|i| {
            digits
                .iter()
                .zip(pattern(i + 1, digits.len()))
                .map(|(a, b)| a * b)
                .sum::<i32>()
                .abs()
                % 10
        })
        .collect()
}

fn pattern(repeat: usize, len: usize) -> Vec<i32> {
    [0, 1, 0, -1]
        .iter()
        .map(|i| iter::repeat(i).take(repeat))
        .flatten()
        .cycle()
        .skip(1)
        .take(len)
        .cloned()
        .collect()
}

#[cfg(test)]
mod tests {
    // No tests in the puzzle text :(
    use super::*;

    #[test]
    fn test_run1() {
        assert_eq!(
            solver1(&digits("80871224585914546619083218645595").unwrap()),
            "24176176"
        );
        assert_eq!(
            solver1(&digits("19617804207202209144916044189917").unwrap()),
            "73745418"
        );
        assert_eq!(
            solver1(&digits("69317163492948606335995924319873").unwrap()),
            "52432133"
        );
    }
}
