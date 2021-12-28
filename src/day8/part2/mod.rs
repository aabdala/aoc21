use std::collections::HashMap;

pub fn solution(input: &str) -> String {
    let sum = input
        .split_terminator('\n')
        .filter(|l| !l.trim().is_empty())
        .map(|line| {
            let line_parts = line.split('|').collect::<Vec<_>>();
            let digit_wirings = *line_parts.get(0).unwrap();
            let digits_str = *line_parts.get(1).unwrap();
            (
                digit_wirings.split_ascii_whitespace().collect::<Vec<_>>(),
                digits_str.split_ascii_whitespace().collect::<Vec<_>>(),
            )
        })
        .fold(0, |acc, (wirings, digits)| acc + solve_line(&wirings, &digits));
    format!("{}", sum)
}

fn solve_line(wirings: &[&str], digits: &[&str]) -> usize {
    let intersects_to_digit: HashMap<[u8; 4], u8> = [
        // key: #s of segment intersects with 1, 4, 7, 8, value: the digit
        ([2, 3, 3, 6], 0),
        ([1, 2, 2, 5], 2),
        ([2, 3, 3, 5], 3),
        ([1, 3, 2, 5], 5),
        ([1, 3, 2, 6], 6),
        ([2, 4, 3, 6], 9),
    ]
    .iter()
    .cloned()
    .collect();

    let mut digit_to_wiring = HashMap::new();
    let mut wiring_to_digit = HashMap::new();
    for wiring in wirings {
        let key = match wiring.len() {
            2 => 1,
            3 => 7,
            4 => 4,
            7 => 8,
            _ => 0,
        };
        if key > 0 {
            digit_to_wiring.insert(key, String::from(*wiring));
            wiring_to_digit.insert(String::from(*wiring), key);
        }
    }
    for wiring in wirings {
        let intersects: [u8; 4] = [1, 4, 7, 8].map(|i| {
            let i_wiring = digit_to_wiring.get(&i).unwrap();
            intersect_count(wiring, i_wiring)
        });
        let decoded_digit = wiring_to_digit
            .get(&String::from(*wiring))
            .or_else(|| Some(intersects_to_digit.get(&intersects).unwrap()));
        wiring_to_digit.insert(String::from(*wiring), *decoded_digit.unwrap());
    }
    digits
        .iter()
        .fold((1000, 0), |(mult, sum), digit_str| {
            (
                mult / 10,
                sum + mult * find_digit(digit_str, &wiring_to_digit),
            )
        })
        .1
}

fn intersect_count(a: &str, b: &str) -> u8 {
    let mut longer = a;
    let mut shorter = b;
    if b.len() > a.len() {
        longer = b;
        shorter = a;
    }
    shorter
        .chars()
        .fold(0, |acc, c| if longer.contains(c) { acc + 1 } else { acc })
}

fn find_digit(digit_str: &str, wiring_to_digit: &HashMap<String, u8>) -> usize {
    // search in the wiring_to_digit map since wirings may have digits out of order :smirk:
    usize::from(
        *wiring_to_digit
            .iter()
            .find(|(key, _)| {
                key.len().max(digit_str.len()) == intersect_count(key, digit_str).into()
            })
            .unwrap()
            .1,
    )
}
