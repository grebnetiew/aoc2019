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
                .enumerate()
                .map(|(j, d)| pattern(i + 1, j) * d)
                .sum::<i32>()
                .abs()
                % 10
        })
        .collect()
}

fn pattern(repeat: usize, i: usize) -> i32 {
    match ((i + 1) / repeat) % 4 {
        0 | 2 => 0,
        1 => 1,
        3 => -1,
        _ => unreachable!(),
    }
}

#[aoc(day16, part2)]
fn solver2(digits: &[i32]) -> String {
    let message_offset: usize = digits
        .iter()
        .take(7)
        .map(i32::to_string)
        .fold(String::from(""), |a, b| a + &b)
        .parse()
        .unwrap();

    let mut digits: Vec<_> = digits
        .iter()
        .cycle()
        .take(10000 * digits.len())
        .cloned()
        .collect();
    for i in 0..100 {
        digits = fft(&digits);
        println!("Did {:?}", i);
    }
    digits
        .iter()
        .skip(message_offset)
        .take(8)
        .map(i32::to_string)
        .fold(String::from(""), |a, b| a + &b)
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
