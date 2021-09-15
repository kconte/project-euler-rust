#![warn(clippy::all)]
#![warn(clippy::pedantic)]

use num_bigint::BigUint;
use num_traits::FromPrimitive;

fn main() {
  run();
}

fn run() {
  let start = std::time::Instant::now();

  // code goes here
  let mut nums = vec![];

  for a in 2..=100 {
    for b in 2..=100 {
      let res = BigUint::from_u64(a).unwrap().pow(b);
      if !nums.contains(&res) {
        nums.push(res);
      }
    }
  }

  let res = nums.len();

  let span = start.elapsed().as_nanos();
  println!("{} {}", res, span);
}
