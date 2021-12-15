mod part1;
mod part2;

use crate::utils;
use std::collections::HashMap;

pub fn solution(input: &str) -> String {
    utils::format_part_solutions(part1::solution(input), part2::solution(input))
}

fn parse_fishbowl(input: &str) -> FishBowl {
    let fishes: Vec<Fish> = input
        .trim_end()
        .split_terminator(',')
        .map(|fish_str| fish_str.parse::<u8>().unwrap())
        .map(Fish::new)
        .collect();
    FishBowl::new(fishes)
}

#[derive(Debug)]
struct Fish {
    life: u8,
}

impl Fish {
    fn new(life: u8) -> Self {
        Fish { life }
    }
}

struct FishBowl {
    fish_map: HashMap<u8, i128>,
}

impl FishBowl {
    fn new(fishes: Vec<Fish>) -> Self {
        let mut fish_map = HashMap::new();
        fishes.iter().for_each(|f| {
            let fishes_entry = fish_map.entry(f.life).or_insert(0);
            *fishes_entry += 1;
        });
        FishBowl { fish_map }
    }

    fn fish_count(&self) -> i128 {
        self.fish_map.values().sum()
    }

    fn tick(&mut self) {
        let to_breed = *self.fish_map.get(&0).unwrap_or(&0);
        let mut new_fishmap: HashMap<u8, i128> = HashMap::new();
        for i in 1..=8 {
            new_fishmap.insert(i - 1, *self.fish_map.get(&i).unwrap_or(&0));
        }
        new_fishmap.insert(8, to_breed);
        let entry = new_fishmap.entry(6).or_insert(0);
        *entry += to_breed;
        self.fish_map = new_fishmap;
    }
}
