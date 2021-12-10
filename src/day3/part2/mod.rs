use crate::day3::calc_decimal;

pub fn solution(input: String) -> String {
    let bit_sizes: Vec<usize> = input
        .split_ascii_whitespace()
        .take(1)
        .map(|line| line.len())
        .collect();
    let bit_size = bit_sizes.get(0).unwrap();

    let input_bits = super::to_bits(input);

    let most_common_bits: Vec<i8> =
        super::count_bits_by_position(super::to_counting_bits(input_bits.clone()))
            .into_iter()
            .map(|b| if b > 0 { 1 } else { 0 })
            .collect();
    let least_common_bits: Vec<i8> = most_common_bits
        .clone()
        .into_iter()
        .map(|b| if b == 0 { 1 } else { 0 })
        .collect();
    let oxygen = calc_rating(bit_size, most_common_bits, input_bits.clone(), 1);
    let co2 = calc_rating(bit_size, least_common_bits, input_bits, 0);

    format!("{}", calc_decimal(oxygen) * calc_decimal(co2))
}

fn calc_rating(
    bit_size: &usize,
    reference_bits: Vec<i8>,
    mut current_set: Vec<Vec<i8>>,
    tiebreaker: i8,
) -> Vec<i8> {
    let mut oxygen = vec![];
    for i in 1..*bit_size {
        let most_common_bits: Vec<i8> = reference_bits.clone().into_iter().take(i).collect();
        current_set = filter_set(current_set.clone(), most_common_bits);
        dbg!(current_set.clone());
        if current_set.len() <= 2 {
            if current_set.len() == 2 {
                let x: Vec<Vec<i8>> = current_set
                    .into_iter()
                    .filter(|each| *each.get(i).unwrap() == tiebreaker)
                    .collect();
                oxygen = x.get(0).unwrap().to_vec();
            } else if current_set.len() == 1 {
                oxygen = current_set.get(0).unwrap().to_vec();
            }
            break;
        }
    }
    oxygen
}

fn filter_set(input: Vec<Vec<i8>>, reference_bits: Vec<i8>) -> Vec<Vec<i8>> {
    input
        .into_iter()
        .filter(|each| {
            let mut each_prefix = each.clone();
            each_prefix.truncate(reference_bits.len());
            each_prefix.eq(&reference_bits)
        })
        .collect()
}
