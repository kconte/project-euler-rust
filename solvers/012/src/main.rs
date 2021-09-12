#![warn(clippy::all)]
#![warn(clippy::pedantic)]

use std::cmp::Ordering;

fn main() {
  run();
}

fn run() {
  let start = std::time::Instant::now();

  // code goes here
  let res = TriangleNum::default()
    .find(|n| num_divisors(*n) > 500)
    .unwrap();

  let span = start.elapsed().as_nanos();
  println!("{} {}", res, span);
}

// calculates the number of positive divisors of a positive integer
fn num_divisors(n: u64) -> u64 {
  let mut count = 0;
  let lim = sqrt(n);

  for i in 1..=lim {
    if n % i == 0 {
      count += 1;
      if i * i != n {
        count += 1;
      }
    }
  }

  count
}

// returns the approximate square root of an integer
#[allow(clippy::similar_names)]
fn sqrt(n: u64) -> u64 {
  let mut min = 0;
  let mut max = n;

  while min < max {
    let mid = (min + max + 1) / 2;
    let square = mid * mid;
    match square.cmp(&n) {
      | Ordering::Equal => return mid,
      | Ordering::Greater => max = mid - 1,
      | Ordering::Less => min = mid,
    }
  }

  min
}

// Simple struct to allow us to iterate over triangle numbers.
#[derive(Default)]
struct TriangleNum {
  idx: u64,
}

impl Iterator for TriangleNum {
  type Item = u64;

  fn next(&mut self) -> Option<Self::Item> {
    let res = self.idx * (self.idx + 1) / 2;
    self.idx += 1;
    Some(res)
  }
}
