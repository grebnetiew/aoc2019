extern crate itertools;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::collections::HashMap;

#[aoc_generator(day6)]
fn orbit_parser(input: &str) -> Vec<(String, String)> {
    input
        .lines()
        .map(|l| {
            l.split(')')
                .map(str::to_string)
                .next_tuple()
                .expect("A line does not contain a valid A)B pair")
        })
        .collect()
}

fn count_depth(hm: &HashMap<String, String>, key: &str) -> usize {
    match hm.get(key) {
        None => 0,
        Some(value) => 1 + count_depth(hm, &value),
    }
}

#[aoc(day6, part1)]
fn solver1(input: &[(String, String)]) -> usize {
    let orbits: HashMap<_, _> = input.iter().map(|(a, b)| (b.clone(), a.clone())).collect();
    let mut total_orbits = 0;
    for k in orbits.keys() {
        total_orbits += count_depth(&orbits, k);
    }
    total_orbits
}

fn ancestors<'a>(hm: &'a HashMap<String, String>, mut key: &'a str) -> Vec<&'a str> {
    let mut ancestors = Vec::new();
    while hm.contains_key(key) {
        key = &hm[key];
        ancestors.push(key);
    }
    ancestors
}

#[aoc(day6, part2)]
fn solver2(input: &[(String, String)]) -> usize {
    let orbits: HashMap<_, _> = input.iter().map(|(a, b)| (b.clone(), a.clone())).collect();
    // Find ancestors of YOU and SAN
    let (you, san) = ("YOU".to_string(), "SAN".to_string());
    let mut anc_you = ancestors(&orbits, &you);
    let mut anc_san = ancestors(&orbits, &san);
    // Remove common ancestors
    while anc_you.last() == anc_san.last() {
        anc_you.pop();
        anc_san.pop();
    }
    // Now these are the diverging paths, minus the common link, plus currently orbited pts
    // Alternatively, these are the exact *edges* that must be transferred
    anc_you.len() + anc_san.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run1() {
        let input = "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L";
        assert_eq!(solver1(&orbit_parser(input)), 42);
    }

    #[test]
    fn test_run2() {
        let input = "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\nK)YOU\nI)SAN";
        assert_eq!(solver2(&orbit_parser(input)), 4);
    }

}
