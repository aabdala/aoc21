use crate::utils::read_input_file;

const BASE2 : i32 = 2;
pub fn solution() {
  part1();
}

fn part2() {
  // TODO
}

fn part1() {
  let bitcount : Vec<i32> = vec![0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];
  let report_analysis = read_input_file("day3")
    .split_ascii_whitespace()
    .map(|each| each.chars().map(|c| if c == '0' { 0 } else { 1 }).collect::<Vec<u8>>())
    .fold((0, bitcount), |(len, bc), linebits| {
      let newbc = linebits.into_iter().zip(bc).map(|(a, b)| b + a as i32).collect();
      return (len + 1, newbc);
    });
  println!("{:?}", report_analysis);


  let gamma = calc_decimal( report_analysis.clone(), |bc, ml| bc > ml);
  let epsilon = calc_decimal( report_analysis, |bc, ml| bc < ml);

  println!("{:?}, {:?}", gamma, epsilon);
  println!("{}", gamma.1 * epsilon.1);
}

fn calc_decimal((len, sums):(i32, Vec<i32>), f: fn(i32, i32) -> bool) -> (u32, i32) {
  let midlen = len / 2;
  sums.into_iter()
  .map(|bitcount| if f(bitcount, midlen) { 1 } else { 0 } )
  .rev()
  .fold((0, 0), |(i, decimal), bit| (i + 1, decimal + bit * BASE2.pow(i)))
}
