use std::collections::HashMap;

pub fn solution(input: &str) -> String {
    let mut digit_count: HashMap<usize, usize> = HashMap::new();
    input
        .split_terminator('\n')
        .flat_map(|line| {
            let line_parts = line.split('|').collect::<Vec<_>>();

            let digits_str = *line_parts.get(1).unwrap();
            digits_str.split_ascii_whitespace().collect::<Vec<_>>()
        })
        .for_each(|digit| {
            let entry = digit_count.entry(digit.len()).or_insert(0);
            *entry += 1;
        });
    let mut unique_count = 0;

    for (key, value) in digit_count {
        if key == 2 || key == 3 || key == 4 || key == 7 {
            unique_count += value;
        }
    }

    format!("{}", unique_count)
}
