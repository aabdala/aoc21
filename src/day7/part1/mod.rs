use std::collections::HashMap;

pub fn solution(input: &str) -> String {
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

    let lr_fuel_costs = calculate_fuel_costs(position_crabcount_list.clone());
    let rl_fuel_costs: Vec<i32> = calculate_fuel_costs(position_crabcount_list.rev())
        .into_iter()
        .rev()
        .collect();

    let total_fuel_costs = lr_fuel_costs.iter().zip(rl_fuel_costs);
    let min_fuel_cost = total_fuel_costs.map(|(lr, rl)| lr + rl).min();

    format!("{}", min_fuel_cost.unwrap())
}

fn calculate_fuel_costs<T>(it: T) -> Vec<i32>
where
    T: Iterator<Item = (i32, i32)>,
{
    let mut result: Vec<i32> = Vec::new();
    it.fold((0, 0, 0), |(crab_acc, last_pos, fuel_acc), (pos, crab_count)| {
        let fuel_at_i = fuel_acc + (pos - last_pos) * crab_acc;
        result.push(fuel_at_i.abs());
        (crab_acc + crab_count, pos, fuel_at_i)
    });
    result
}
