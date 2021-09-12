#![warn(clippy::all)]
#![warn(clippy::pedantic)]

fn main() {
  run();
}

fn run() {
  let start = std::time::Instant::now();

  // code goes here
  let res = (2 * 2 * 2 * 2) * (3 * 3) * 5 * 7 * 11 * 13 * 17 * 19;

  let span = start.elapsed().as_nanos();
  println!("{} {}", res, span);
}
