mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod utils;

fn main() {
    println!("day 1:\n{}", day1::solution(&utils::read_input_file("day1")));
    println!("day 2:\n{}", day2::solution(&utils::read_input_file("day2")));
    println!("day 3:\n{}", day3::solution(&utils::read_input_file("day3")));
    println!("day 4:\n{}", day4::solution(&utils::read_input_file("day4")));
    println!("day 5:\n{}", day5::solution(&utils::read_input_file("day5")));
    println!("day 6:\n{}", day6::solution(&utils::read_input_file("day6")));
    println!("day 7:\n{}", day7::solution(&utils::read_input_file("day7")));
}
