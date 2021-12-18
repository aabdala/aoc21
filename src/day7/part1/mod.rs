pub fn solution(input: &str) -> String {
    format!("{}", super::solve(input, calculate_fuel_costs))
}

fn calculate_fuel_costs(it: &mut dyn Iterator<Item = (i32, i32)>) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    it.fold(
        (0, 0, 0),
        |(crab_acc, last_pos, fuel_acc), (pos, crab_count)| {
            let fuel_at_i = fuel_acc + (pos - last_pos) * crab_acc;
            result.push(fuel_at_i.abs());
            (crab_acc + crab_count, pos, fuel_at_i)
        },
    );
    result
}
