mod day1;
mod day2;
mod day3;
mod day4;
mod utils;

fn main() {
    println!("day 1:\n{}", day1::solution(&utils::read_input_file("day1")));
    println!("day 2:\n{}", day2::solution(&utils::read_input_file("day2")));
    println!("day 3:\n{}", day3::solution(&utils::read_input_file("day3")));
    println!("day 4:\n{}", day4::solution(&utils::read_input_file("day4")));
}
