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
  let upper = BigUint::from_u64(40).unwrap().factorial();
  let lower = BigUint::from_u64(20).unwrap().factorial();
  let lower = lower.clone() * lower;
  let res = upper / lower;

  let span = start.elapsed().as_nanos();
  println!("{} {}", res, span);
}
