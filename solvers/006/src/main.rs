#![warn(clippy::all)]
#![warn(clippy::pedantic)]

fn main() {
  run();
}

fn run() {
  let start = std::time::Instant::now();

  // code goes here
  let res;

  let square_sum: u64 = (1..=100).map(|i| i * i).sum();
  let mut sum_square: u64 = (100 * 101) / 2;
  sum_square *= sum_square;

  res = sum_square - square_sum;

  let span = start.elapsed().as_nanos();
  println!("{} {}", res, span);
}
