use aoc_runner_derive::{aoc, aoc_generator};
use std::num::ParseIntError;

#[aoc_generator(day16)]
fn digits(input: &str) -> Result<Vec<i32>, ParseIntError> {
    input.chars().map(|c| c.to_string().parse()).collect()
}

#[aoc(day16, part1)]
fn solver1(digits: &[i32]) -> i32 {
    let mut digits = digits.to_vec();
    for _ in 0..100 {
        digits = fft(&digits);
    }
    digits.iter().take(8).fold(0, |acc, x| 10 * acc + x)
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
fn solver2(digits: &[i32]) -> i32 {
    // We use the fact that for our input, the length of the true input
    // (6 million something) is not much larger than the message offset
    // (5 million something). In that case, there's only ones.

    let message_offset = digits.iter().take(7).fold(0, |acc, x| 10 * acc + x) as usize;
    let desired_length_after_offset = digits.len() * 10000 - message_offset;

    let important_digits: Vec<i32> = digits
        .iter()
        .cycle()
        .skip(message_offset % digits.len())
        .take(desired_length_after_offset)
        .cloned()
        .collect();
    let mut reversed_important_digits: Vec<i32> = important_digits.iter().rev().cloned().collect();

    for _ in 0..100 {
        reversed_important_digits = fft_sneaky(&reversed_important_digits);
    }

    reversed_important_digits
        .iter()
        .rev()
        .take(8)
        .fold(0, |acc, x| 10 * acc + x)
}

fn fft_sneaky(digits: &[i32]) -> Vec<i32> {
    // Give this function only the digits after skipping <offset> digits,
    // in reverse order!
    digits
        .iter()
        .scan(0, |acc, x| {
            *acc += x;
            Some(*acc % 10)
        })
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
            24_176_176
        );
        assert_eq!(
            solver1(&digits("19617804207202209144916044189917").unwrap()),
            73_745_418
        );
        assert_eq!(
            solver1(&digits("69317163492948606335995924319873").unwrap()),
            52_432_133
        );
    }

    #[test]
    fn test_run2() {
        assert_eq!(
            solver2(&digits("03036732577212944063491565474664").unwrap()),
            84_462_026
        );
        assert_eq!(
            solver2(&digits("02935109699940807407585447034323").unwrap()),
            78_725_270
        );
        assert_eq!(
            solver2(&digits("03081770884921959731165446850517").unwrap()),
            53_553_731
        );
    }
}
