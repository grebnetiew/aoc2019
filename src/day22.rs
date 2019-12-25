use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;
use std::num::ParseIntError;

#[aoc_generator(day22)]
fn shuffles(input: &str) -> Result<Vec<Shuffle>, ParseIntError> {
    let re_cut = Regex::new(r"cut (-?[0-9]+)").unwrap();
    let re_din = Regex::new(r"deal with increment (-?[0-9]+)").unwrap();
    let re_rev = Regex::new(r"deal into new stack").unwrap();
    input
        .lines()
        .map(|line| -> Result<Shuffle, ParseIntError> {
            if let Some(caps) = re_cut.captures(line) {
                return Ok(Shuffle::Cut(caps[1].parse()?));
            }
            if let Some(caps) = re_din.captures(line) {
                return Ok(Shuffle::DealIncrement(caps[1].parse()?));
            }
            if let Some(_caps) = re_rev.captures(line) {
                return Ok(Shuffle::Reverse);
            }
            panic!("Can't parse {:?}", line);
        })
        .collect()
}

#[derive(Debug)]
enum Shuffle {
    Cut(i64),
    DealIncrement(i64),
    Reverse,
}

#[aoc(day22, part1)]
fn solver1(input: &[Shuffle]) -> i64 {
    shuffle(input, 10007, 2019)
}

fn shuffle(input: &[Shuffle], len: i64, find: i64) -> i64 {
    let mut start = 0i64;
    let mut stride = 1i64;

    for s in input.iter() {
        match s {
            Shuffle::Cut(n) => start = (start + (n - 1) * stride + len) % len,
            Shuffle::DealIncrement(n) => {
                stride *= -n;
                start -= stride;
            }
            Shuffle::Reverse => {
                stride *= -1;
                start = len - start;
            }
        }
        print_cards(start, stride, len);
    }
    let mut i = start;
    let mut steps = 0;
    while steps != len {
        i = (i + stride + len) % len;
        if i == find {
            return steps;
        }
        steps += 1;
    }
    0
}

fn print_cards(start: i64, stride: i64, len: i64) {
    let mut i = start;
    let mut steps = 0;
    while steps != len {
        steps += 1;
        i = (i + stride + len) % len;
        print!("{} ", i)
    }
    println!();
}

#[aoc(day22, part2)]
fn solver2(input: &[Shuffle]) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basics() {
        assert_eq!(shuffle(&shuffles("deal into new stack").unwrap(), 10, 4), 5);
        assert_eq!(shuffle(&shuffles("cut 3").unwrap(), 10, 4), 1);
        assert_eq!(shuffle(&shuffles("cut -4").unwrap(), 10, 4), 8);
        assert_eq!(
            shuffle(&shuffles("deal with increment 3").unwrap(), 10, 4),
            2
        );
    }

    #[test]
    fn test_run1() {
        assert_eq!(
            shuffle(
                &shuffles(
                    "deal with increment 7\n\
                     deal into new stack\n\
                     deal into new stack"
                )
                .unwrap(),
                10,
                4
            ),
            8
        );
        assert_eq!(
            shuffle(
                &shuffles(
                    "cut 6\n\
                     deal with increment 7\n\
                     deal into new stack"
                )
                .unwrap(),
                10,
                4
            ),
            3
        );
        assert_eq!(
            shuffle(
                &shuffles(
                    "deal with increment 7\n\
                     deal with increment 9\n\
                     cut -2"
                )
                .unwrap(),
                10,
                4
            ),
            4
        );
        assert_eq!(
            shuffle(
                &shuffles(
                    "deal into new stack\n\
                     cut -2\n\
                     deal with increment 7\n\
                     cut 8\n\
                     cut -4\n\
                     deal with increment 7\n\
                     cut 3\n\
                     deal with increment 9\n\
                     deal with increment 3\n\
                     cut -1"
                )
                .unwrap(),
                10,
                4
            ),
            5
        );
    }
}
