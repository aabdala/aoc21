use core::panic;

use crate::utils;

enum Instruction {
    UP(i32), DOWN(i32), FORWARD(i32)
}
pub fn solution() {
  part2();
}

fn part2() {
  let contents = utils::read_input_file("day2");
  let position = contents.split_terminator("\n")
    .map(|each| {
      let elems : Vec<&str> = each.split_ascii_whitespace().collect();
      let value = elems.get(1).unwrap().parse::<i32>().unwrap();
      match *elems.get(0).unwrap() {
        "up" => Instruction::UP(value),
        "down" => Instruction::DOWN(value),
        "forward" => Instruction::FORWARD(value),
        _ => panic!("unexpected command")
      }
    })
    .fold((0 ,0, 0), |position, inst| match inst {
        Instruction::DOWN(units) => (position.0 + units, position.1, position.2),
        Instruction::UP(units) => (position.0 - units, position.1, position.2),
        Instruction::FORWARD(units) => (position.0, position.1 + units, position.2 + position.0 * units),
      }
    );
    println!("{}", position.1 * position.2);
}

fn part1() {
  let contents = utils::read_input_file("day2");
  let position = contents.split_terminator("\n")
    .map(|each| {
      let elems : Vec<&str> = each.split_ascii_whitespace().collect();
      let value = elems.get(1).unwrap().parse::<i32>().unwrap();
      match *elems.get(0).unwrap() {
        "up" => Instruction::UP(value),
        "down" => Instruction::DOWN(value),
        "forward" => Instruction::FORWARD(value),
        _ => panic!("unexpected command")
      }
    })
    .fold((0, 0), |position, inst| match inst {
        Instruction::DOWN(units) => (position.0 + units, position.1),
        Instruction::UP(units) => (position.0 - units, position.1),
        Instruction::FORWARD(units) => (position.0, position.1 + units),
      }
    );
    println!("{}", position.0 * position.1);
}
