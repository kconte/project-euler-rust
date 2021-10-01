#![warn(clippy::all)]
#![warn(clippy::pedantic)]

mod utils;

use utils::{sieve, FromDigits, ToDigits};

fn main() {
  run();
}

fn run() {
  let start = std::time::Instant::now();

  // code goes here
  let primes = sieve(1_000_000);

  let res = primes.iter().filter(|&&p| is_circular(p, &primes)).count();

  let span = start.elapsed().as_nanos();
  println!("{} {}", res, span);
}

fn is_circular(val: u64, primes: &[u64]) -> bool {
  let digits: Vec<u64> = val.to_digits(10);

  let mut buffer: Vec<u64> = vec![];

  for i in 1..digits.len() {
    for j in 0..digits.len() {
      buffer.push(digits[(i + j) % digits.len()]);
    }
    let rotation = u64::from_digits(&mut buffer, 10);
    if !primes.contains(&rotation) {
      return false;
    }
  }

  true
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn is_circular_works() {
    let primes = sieve(1_000_000);
    assert!(is_circular(197, &primes));
    assert!(!is_circular(23, &primes));
  }
}
