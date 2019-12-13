use aoc_runner_derive::{aoc, aoc_generator};
use std::fmt;

#[aoc_generator(day8)]
fn one_line_many_numbers(input: &str) -> Vec<u32> {
    input
        .chars()
        .map(|n| n.to_digit(10).expect("Found a non-base-ten digit"))
        .collect()
}

struct Sif {
    pixels: Vec<u32>,
    width: u32,
    height: u32,
}

impl Sif {
    fn from_raw_pixels(pixels: Vec<u32>, width: u32, height: u32) -> Self {
        assert!(pixels.len() as u32 >= width * height);
        Self {
            pixels,
            width,
            height,
        }
    }

    fn num_layers(&self) -> u32 {
        self.pixels.len() as u32 / (self.width * self.height)
    }

    fn px_unchecked(&self, layer: u32, x: u32, y: u32) -> u32 {
        self.pixels[(layer * (self.width * self.height) + y * self.width + x) as usize]
    }
}

impl fmt::Display for Sif {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        writeln!(f)?;

        for y in 0..self.height {
            for x in 0..self.width {
                let mut color = 0;
                for lay in 0..self.num_layers() {
                    color = self.px_unchecked(lay, x, y);
                    if color != 2 {
                        break;
                    }
                }
                write!(f, "{}", if color == 1 { "#" } else { " " })?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[aoc(day8, part1)]
fn solver1(input: &[u32]) -> u32 {
    let image = Sif::from_raw_pixels(input.to_vec(), 25, 6);
    let mut min = u32::max_value();
    let mut answer_for_argmin = 0;

    for i in 0..image.num_layers() {
        let mut counts = [0, 0, 0];
        for px in 0..150 {
            // Abuse my own api :P
            counts[image.px_unchecked(i, px, 0) as usize] += 1;
        }
        if counts[0] < min {
            min = counts[0];
            answer_for_argmin = counts[1] * counts[2];
        }
    }

    // Argmin contains the fewest zeros
    answer_for_argmin
}

#[aoc(day8, part2)]
fn solver2(input: &[u32]) -> String {
    let image = Sif::from_raw_pixels(input.to_vec(), 25, 6);
    image.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run1() {
        assert_eq!(
            Sif::from_raw_pixels(one_line_many_numbers("123456789012"), 3, 2).px_unchecked(1, 1, 1),
            1
        );
    }

    #[test]
    fn test_run2() {
        assert_eq!(
            Sif::from_raw_pixels(one_line_many_numbers("0222112222120000"), 2, 2).to_string(),
            "\n #\n# \n"
        );
    }

}
