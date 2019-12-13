use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day4)]
fn number_range(input: &str) -> Vec<Vec<u8>> {
    input.split('-').map(|s| s.as_bytes().to_vec()).collect()
}

// The dumb/iterative method works. I am on a time constraint and might do a cerebral
// solution later on :)
#[aoc(day4, part1, Hadouken)]
fn solver1(input: &[Vec<u8>]) -> u32 {
    let nine = b'9';
    let mut count = 0;
    for i1 in input[0][0]..input[1][0] {
        for i2 in i1..=nine {
            for i3 in i2..=nine {
                for i4 in i3..=nine {
                    for i5 in i4..=nine {
                        for i6 in i5..=nine {
                            let current_numbers = &[i1, i2, i3, i4, i5, i6];
                            if (i1 == i2 || i2 == i3 || i3 == i4 || i4 == i5 || i5 == i6)
                                && number_from(&input[0]) < number_from(current_numbers)
                                && number_from(current_numbers) < number_from(&input[1])
                            {
                                count += 1;
                            }
                        }
                    }
                }
            }
        }
    }
    count
}

#[aoc(day4, part2, Hadouken)]
fn solver2(input: &[Vec<u8>]) -> u32 {
    let nine = b'9';
    let mut count = 0;
    for i1 in input[0][0]..input[1][0] {
        for i2 in i1..=nine {
            for i3 in i2..=nine {
                for i4 in i3..=nine {
                    for i5 in i4..=nine {
                        for i6 in i5..=nine {
                            let current_numbers = &[i1, i2, i3, i4, i5, i6];
                            if doubles_constraint(current_numbers)
                                && number_from(&input[0]) < number_from(current_numbers)
                                && number_from(current_numbers) < number_from(&input[1])
                            {
                                count += 1;
                            }
                        }
                    }
                }
            }
        }
    }
    count
}

fn number_from(digits: &[u8]) -> u32 {
    let mut result = 0;
    for &digit in digits {
        result *= 10;
        result += digit as u32;
    }
    result
}

fn doubles_constraint(digits: &[u8]) -> bool {
    (digits[0] == digits[1] && digits[1] != digits[2])
        || (digits[0] != digits[1] && digits[1] == digits[2] && digits[2] != digits[3])
        || (digits[1] != digits[2] && digits[2] == digits[3] && digits[3] != digits[4])
        || (digits[2] != digits[3] && digits[3] == digits[4] && digits[4] != digits[5])
        || (digits[3] != digits[4] && digits[4] == digits[5])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_from() {
        assert_eq!(13579, number_from(&[1, 3, 5, 7, 9]));
    }
}
