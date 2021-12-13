use crate::day3::calc_decimal;

enum Rating {
    Oxygen,
    Co2,
}

impl Rating {
    fn tiebreaker(&self) -> i8 {
        match self {
            Self::Oxygen => 1,
            Self::Co2 => 0,
        }
    }

    fn bitcount_to_bit(&self, bitcount: i32) -> i8 {
        if bitcount == 0 {
            self.tiebreaker()
        } else {
            match self {
                Self::Oxygen => {
                    if bitcount > 0 {
                        1
                    } else {
                        0
                    }
                }
                Self::Co2 => {
                    if bitcount > 0 {
                        0
                    } else {
                        1
                    }
                }
            }
        }
    }
}

pub fn solution(input: &str) -> String {
    let bit_size = input.chars().position(|c| c == '\n').unwrap();

    let input_bits = super::to_bits(input);

    let oxygen = calc_rating(bit_size, &input_bits, Rating::Oxygen);
    let co2 = calc_rating(bit_size, &input_bits, Rating::Co2);

    format!("{}", calc_decimal(&oxygen) * calc_decimal(&co2))
}

fn calc_rating(bit_size: usize, input: &[Vec<i8>], rating: Rating) -> Vec<i8> {
    let mut current_set = input.to_owned();
    for i in 1..bit_size+1 {
        let reference_bits: Vec<i8> =
            super::count_bits_by_position(&super::to_counting_bits(&current_set))
                .into_iter()
                .map(|bc| rating.bitcount_to_bit(bc))
                .take(i)
                .collect();
        current_set = filter_set(&current_set, &reference_bits);
        if current_set.len() < 2 {
            break;
        }
    }
    current_set.get(0).unwrap().to_vec()
}

fn filter_set(input: &[Vec<i8>], reference_bits: &[i8]) -> Vec<Vec<i8>> {
    let reference_last = reference_bits.last().unwrap();
    input
        .iter()
        .filter(|each| each.get(reference_bits.len() - 1).unwrap() == reference_last)
        .map(|e| e.to_owned())
        .collect()
}
