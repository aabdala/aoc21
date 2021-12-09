pub fn solution(input: String) -> String {
    let most_common_bits: Vec<u8> = super::count_bits_by_position(input.clone())
        .into_iter()
        .map(|b| if b > 0 { 1 } else { 0 })
        .collect();
    let least_common_bits = most_common_bits.clone().into_iter().map(|b| if b == 0 { 1 } else { 0 }).collect();

    let oxygen = calc_rating(input.clone(), most_common_bits);
    let co2 = calc_rating(input, least_common_bits);

    format!("{}", oxygen * co2)
}

fn calc_rating(input: String, reference_bits: Vec<u8>) -> i32 {
    let best_match: Vec<u8> = vec![];
    let bits = input
        .split_ascii_whitespace()
        .map(|each| {
            each.chars()
                .map(|c| if c == '0' { 0 } else { 1 })
                .collect::<Vec<u8>>()
        })
        .fold((0, best_match), |(match_count, best_match), each| {
            let mut each_match_count = 0;
            for (i, bit) in each.clone().into_iter().enumerate() {
                if bit == *reference_bits.get(i).unwrap() {
                    each_match_count += 1;
                } else {
                    break;
                }
            }

            if each_match_count >= match_count {
                (each_match_count, each)
            } else {
                (match_count, best_match)
            }
        }).1;
    super::calc_decimal(bits)
}
