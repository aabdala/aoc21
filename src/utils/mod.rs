use std::fs;

pub fn read_example_file(day: &str) -> String {
    read_file(day, "example")
}

pub fn read_input_file(day: &str) -> String {
    read_file(day, "input")
}

fn read_file(day: &str, name: &str) -> String {
    let filename = format!("src/{}/{}", day, name);
    fs::read_to_string(filename).expect("Something went wrong reading the file")
}

pub fn format_part_solutions(part1_sol: String, part2_sol: String) -> String {
    format!("\tpart 1: {}\n\tpart 2: {}\n", part1_sol, part2_sol)
}
