#![warn(clippy::all)]
#![warn(clippy::pedantic)]

use std::fs;

fn main() {
  run();
}

#[allow(clippy::cast_lossless)]
#[allow(clippy::cast_possible_truncation)]
fn run() {
  let start = std::time::Instant::now();

  // code goes here
  let mut names: Vec<String> = fs::read_to_string("./names.txt")
    .expect("could not read file")
    .replace("\"", "")
    .replace(",", " ")
    .split_ascii_whitespace()
    .map(std::string::ToString::to_string)
    .collect();
  names.sort();

  let res: usize = names
    .iter()
    .enumerate()
    .map(|(i, name)| {
      name
        .chars()
        .map(|c| ((c as u8) - b'A' + 1) as usize)
        .sum::<usize>()
        * (i + 1)
    })
    .sum();

  let span = start.elapsed().as_nanos();
  println!("{} {}", res, span);
}
