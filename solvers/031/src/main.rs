#![warn(clippy::all)]
#![warn(clippy::pedantic)]

fn main() {
  run();
}

fn run() {
  let start = std::time::Instant::now();

  // code goes here
  let res = recurse(0, 0);

  let span = start
    .elapsed()
    .as_nanos();
  println!("{} {}", res, span);
}

const COINS: &[u64] = &[1, 2, 5, 10, 20, 50, 100, 200];
const GOAL: u64 = 200;

fn recurse(sum: u64, idx: usize) -> u64 {
  if sum > GOAL || idx >= COINS.len() {
    0
  } else if sum == GOAL {
    1
  } else {
    recurse(sum + COINS[idx], idx) + recurse(sum, idx + 1)
  }
}
