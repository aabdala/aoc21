pub fn solution(input: String) -> String {
    let report_analysis = super::count_bits_by_position(input);

    let gamma = calc_rating(report_analysis.clone(), |bc| bc > 0);
    let epsilon = calc_rating(report_analysis, |bc| bc < 0);

    format!("{}", gamma * epsilon)
}

fn calc_rating(bitcounts: Vec<i32>, f: fn(i32) -> bool) -> i32 {
    let binary : Vec<u8> = bitcounts
        .into_iter()
        .map(|bitcount| if f(bitcount) { 1 } else { 0 })
        .collect();
    super::calc_decimal(binary)
}

