use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;
use std::cmp::Ordering;
use std::collections::HashMap;

#[aoc_generator(day14)]
fn reactions(input: &str) -> HashMap<String, (usize, Vec<Ingredient>)> {
    let re_full =
        Regex::new(r"(?P<in>((\d+) ([A-Z]+),? )+)=> (?P<nout>\d+) (?P<sout>[A-Z]+)").unwrap();
    let re_in = Regex::new(r"((?P<nin>\d+) (?P<sin>[A-Z]+),? )").unwrap();
    input
        .lines()
        .map(|line| {
            let full_caps = re_full
                .captures(line)
                .expect("A line did not match the full regex");
            let ingredients = &full_caps["in"];
            let nout = full_caps["nout"].parse().unwrap();
            let sout = full_caps["sout"].to_owned();

            let ingredients = re_in
                .captures_iter(&ingredients)
                .map(|caps| Ingredient {
                    amount: caps["nin"].parse().unwrap(),
                    name: caps["sin"].to_owned(),
                })
                .collect();
            (sout, (nout, ingredients))
        })
        .collect()
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Ingredient {
    amount: usize,
    name: String,
}

fn reactor(reactions: &HashMap<String, (usize, Vec<Ingredient>)>, want_fuel: usize) -> usize {
    let mut needed = vec![Ingredient {
        amount: want_fuel,
        name: "FUEL".to_owned(),
    }];

    let mut leftovers = Vec::new();

    while let Some(idx) = non_ore(&needed) {
        let mut requirement = needed.remove(idx);

        // First check if we have this in leftovers
        if let Some(leftover_idx) = find_material(&leftovers, &requirement.name) {
            let mut leftover = leftovers.remove(leftover_idx);
            match requirement.amount.cmp(&leftover.amount) {
                Ordering::Less => {
                    // Requirement satisfied by leftovers
                    leftover.amount -= requirement.amount;
                    leftovers.push(leftover);
                    continue;
                }
                Ordering::Greater => {
                    // We use the leftovers but need more
                    requirement.amount -= leftover.amount;
                }
                Ordering::Equal => {
                    // Exactly enough!
                    continue;
                }
            }
        }

        // We still need more. Find the reaction
        let reaction = reactions
            .get(&requirement.name)
            .unwrap_or_else(|| panic!("No way to get {}!", requirement.name));

        // This is division of req.amount / reac.0 but rounding up
        let how_often = (requirement.amount + reaction.0 - 1) / reaction.0;

        for ingredient in &reaction.1 {
            let mut ingredient = ingredient.clone();
            ingredient.amount *= how_often;

            needed.push(ingredient);
        }

        let amount_leftover = how_often * reaction.0 - requirement.amount;
        if amount_leftover > 0 {
            leftovers.push(Ingredient {
                amount: amount_leftover,
                name: requirement.name,
            })
        }
    }
    needed.iter().map(|i| i.amount).sum()
}

#[aoc(day14, part1)]
fn solver1(reactions: &HashMap<String, (usize, Vec<Ingredient>)>) -> usize {
    reactor(reactions, 1)
}

#[aoc(day14, part2)]
fn solver2(reactions: &HashMap<String, (usize, Vec<Ingredient>)>) -> usize {
    let mut want_fuel = 1;
    loop {
        let ore_taken = reactor(reactions, want_fuel);
        if ore_taken > 1_000_000_000_000 {
            break;
        }
        want_fuel *= 2;
    }

    // Binary search
    want_fuel /= 2;
    let mut step = want_fuel / 2;
    loop {
        let ore_taken = reactor(reactions, want_fuel);

        if ore_taken > 1_000_000_000_000 {
            want_fuel -= step;
            if step == 1 && reactor(reactions, want_fuel) <= 1_000_000_000_000 {
                break want_fuel;
            }
        } else {
            want_fuel += step;
        }

        step = if step / 2 == 0 { 1 } else { step / 2 };
    }
}

fn non_ore(ing: &[Ingredient]) -> Option<usize> {
    for (idx, material) in ing.iter().enumerate() {
        if material.name != "ORE" {
            return Some(idx);
        }
    }
    None
}

fn find_material(ing: &[Ingredient], mtl: &str) -> Option<usize> {
    for (idx, material) in ing.iter().enumerate() {
        if material.name == mtl {
            return Some(idx);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run1() {
        assert_eq!(solver1(&reactions("10 ORE => 10 A\n1 ORE => 1 B\n7 A, 1 B => 1 C\n7 A, 1 C => 1 D\n7 A, 1 D => 1 E\n7 A, 1 E => 1 FUEL")), 31);
        assert_eq!(solver1(&reactions("9 ORE => 2 A\n8 ORE => 3 B\n7 ORE => 5 C\n3 A, 4 B => 1 AB\n5 B, 7 C => 1 BC\n4 C, 1 A => 1 CA\n2 AB, 3 BC, 4 CA => 1 FUEL")), 165);
        assert_eq!(
            solver1(&reactions(
                "157 ORE => 5 NZVS
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
179 ORE => 7 PSHF
177 ORE => 5 HKGWZ
7 DCFZ, 7 PSHF => 2 XJWVT
165 ORE => 2 GPVTF
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT"
            )),
            13312
        );
    }

    #[test]
    fn test_run2() {
        assert_eq!(
            solver2(&reactions(
                "157 ORE => 5 NZVS
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
179 ORE => 7 PSHF
177 ORE => 5 HKGWZ
7 DCFZ, 7 PSHF => 2 XJWVT
165 ORE => 2 GPVTF
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT"
            )),
            82_892_753
        );
        assert_eq!(
            solver2(&reactions(
                "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
17 NVRVD, 3 JNWZP => 8 VPVL
53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
22 VJHF, 37 MNCFX => 5 FWMGM
139 ORE => 4 NVRVD
144 ORE => 7 JNWZP
5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
145 ORE => 6 MNCFX
1 NVRVD => 8 CXFTF
1 VJHF, 6 MNCFX => 4 RFSQX
176 ORE => 6 VJHF"
            )),
            5_586_022
        );
        assert_eq!(
            solver2(&reactions(
                "171 ORE => 8 CNZTR
7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
114 ORE => 4 BHXH
14 VRPVC => 6 BMBT
6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
5 BMBT => 4 WPTQ
189 ORE => 9 KTJDG
1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
12 VRPVC, 27 CNZTR => 2 XDBXC
15 KTJDG, 12 BHXH => 5 XCVML
3 BHXH, 2 VRPVC => 7 MZWV
121 ORE => 7 VRPVC
7 XCVML => 6 RJRHP
5 BHXH, 4 VRPVC => 5 LTCX"
            )),
            460_664
        );
    }
}
