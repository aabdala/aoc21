use crate::utils;

pub fn solution() -> String {
    let contents = utils::read_input_file("day1");
    let mut incs = 0;

    contents
        .split_ascii_whitespace()
        .map(|elem| elem.parse::<i32>().unwrap())
        .fold(0, |last, curr| {
            if curr > last {
                incs += 1;
            }
            curr
        });
    format!("{}", incs)
}
