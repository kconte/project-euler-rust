#![warn(clippy::all)]
#![warn(clippy::pedantic)]

use factorial::Factorial;
use num_bigint::BigUint;
use num_traits::FromPrimitive;

fn main() {
  run();
}

fn run() {
  let start = std::time::Instant::now();

  // code goes here
  let res: u32 = BigUint::from_u64(100)
    .expect("could not parse BigUint from u64")
    .factorial()
    .to_string()
    .chars()
    .filter_map(|c| c.to_digit(10))
    .sum();

  let span = start.elapsed().as_nanos();
  println!("{} {}", res, span);
}
