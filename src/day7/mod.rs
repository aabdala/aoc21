mod part1;
mod part2;

use crate::utils;
use std::collections::HashMap;

pub fn solution(input: &str) -> String {
    utils::format_part_solutions(part1::solution(input), part2::solution(input))
}

fn solve(input: &str, cost_fn: fn(&mut dyn Iterator<Item = (i32, i32)>) -> Vec<(i32)>) -> i32
{
    let mut crab_positions: HashMap<i32, i32> = HashMap::new();
    let mut min_pos = i32::MAX;
    let mut max_pos = 0;
    input
        .split_terminator(',')
        .map(|e| e.trim().parse::<i32>().expect("not a number"))
        .for_each(|crab_pos| {
            if crab_pos < min_pos {
                min_pos = crab_pos;
            }
            if crab_pos > max_pos {
                max_pos = crab_pos;
            }
            let crab_pos_count = crab_positions.entry(crab_pos).or_insert(0);
            *crab_pos_count += 1
        });

    let position_crabcount_list =
        (min_pos..=max_pos).map(|pos| (pos, *crab_positions.get(&pos).unwrap_or(&0)));

    let lr_fuel_costs = cost_fn(&mut position_crabcount_list.clone());
    let rl_fuel_costs: Vec<i32> = cost_fn(&mut position_crabcount_list.rev())
        .into_iter()
        .rev()
        .collect();

    let total_fuel_costs = lr_fuel_costs.iter().zip(rl_fuel_costs);
    total_fuel_costs.map(|(lr, rl)| lr + rl).min().unwrap()
}
