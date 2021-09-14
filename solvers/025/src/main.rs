#![warn(clippy::all)]
#![warn(clippy::pedantic)]

use num_bigint::BigUint;
use num_traits::{One, Zero};

fn main() {
  run();
}

fn run() {
  let start = std::time::Instant::now();

  // code goes here
  let limit = "9"
    .repeat(999)
    .parse::<BigUint>()
    .expect("Could not parse BigUint");

  let res = Fibonacci::default()
    .enumerate()
    .take_while(|(_, v)| *v <= limit)
    .count()
    + 1;

  let span = start.elapsed().as_nanos();
  println!("{} {}", res, span);
}

struct Fibonacci {
  a: BigUint,
  b: BigUint,
}

impl Default for Fibonacci {
  fn default() -> Self {
    Self {
      a: Zero::zero(),
      b: One::one(),
    }
  }
}

impl Iterator for Fibonacci {
  type Item = BigUint;

  fn next(&mut self) -> Option<Self::Item> {
    let next = self.a.clone() + self.b.clone();
    self.a = self.b.clone();
    self.b = next.clone();
    Some(self.a.clone())
  }
}
