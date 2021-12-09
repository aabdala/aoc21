use std::fs;

pub fn read_input_file(day: &str) -> String {
  let filename = "src/".to_owned() + day + "/input";
  let contents = fs::read_to_string(filename)
      .expect("Something went wrong reading the file");
  return contents;
}
