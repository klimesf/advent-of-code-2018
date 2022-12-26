use std::collections::{HashSet};
use std::fs;

use regex::Regex;

pub(crate) fn day24() {
    let input = fs::read_to_string("input/day24/test.txt").unwrap();
    let re = Regex::new(
        r"^(\d+) units each with (\d+) hit points \((.+)\) with an attack that does (\d+) (\w+) damage at initiative (\d+)$"
    ).unwrap();

    let mut groups = vec!();
    let mut current_loyalty = Loyalty::ImmuneSystem;
    for row in input.trim().split("\n") {
        if row == "Immune System:" || row == "" {
            continue;
        }
        if row == "Infection:" {
            current_loyalty = Loyalty::Infection;
            continue;
        }

        let c = re.captures(row).unwrap();
        let units = c.get(1).unwrap().as_str().parse().unwrap();
        let hit_points = c.get(2).unwrap().as_str().parse().unwrap();
        let weakness = HashSet::new(); // TODO
        let attack_damage = c.get(4).unwrap().as_str().parse().unwrap();
        let attack_type = DamageType::Slashing; // TODO
        let initiative = c.get(6).unwrap().as_str().parse().unwrap();

        groups.push(Group {
            units,
            hit_points,
            attack_damage,
            attack_type,
            weakness: HashSet::new(),
            initiative,
            loyalty: current_loyalty.clone(),
        })
    }

    println!("{:?}", groups);
}

#[derive(Debug)]
enum DamageType {
    Slashing,
    Bludgeoning,
    Fire,
    Cold,
    Radiation,
}

#[derive(Copy, Clone, Debug)]
enum Loyalty {
    ImmuneSystem,
    Infection,
}

#[derive(Debug)]
struct Group {
    units: usize,
    hit_points: usize,
    attack_damage: usize,
    attack_type: DamageType,
    weakness: HashSet<DamageType>,
    initiative: usize,
    loyalty: Loyalty,
}

impl Group {
    fn get_effective_power(&self) -> usize {
        self.units * self.attack_damage
    }
}

#[cfg(test)]
mod day24_tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
