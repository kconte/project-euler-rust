#![warn(clippy::all)]
#![warn(clippy::pedantic)]

fn main() {
  run();
}

fn run() {
  let start = std::time::Instant::now();

  // code goes here
  let limit: u64 = 999;
  let mut res = 0;
  let mut soft_limit;

  soft_limit = limit / 3;
  res += 3 * (soft_limit * (soft_limit + 1)) / 2;

  soft_limit = limit / 5;
  res += 5 * (soft_limit * (soft_limit + 1)) / 2;

  soft_limit = limit / 15;
  res -= 15 * (soft_limit * (soft_limit + 1)) / 2;

  let span = start.elapsed().as_nanos();
  println!("{} {}", res, span);
}
