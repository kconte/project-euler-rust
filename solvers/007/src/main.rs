#![warn(clippy::all)]
#![warn(clippy::pedantic)]

fn main() {
  run();
}

fn run() {
  let start = std::time::Instant::now();

  // code goes here
  let res = (0..).filter(|v| v.is_prime()).nth(10000).unwrap();

  let span = start.elapsed().as_nanos();
  println!("{} {}", res, span);
}

trait IsPrime {
  fn is_prime(&self) -> bool;
}

impl IsPrime for u64 {
  fn is_prime(&self) -> bool {
    let val = *self;

    if val == 2 {
      true
    } else if val < 2 || val % 2 == 0 {
      false
    } else {
      let mut i = 3;
      while i * i <= val {
        if val % i == 0 {
          return false;
        }
        i += 2;
      }
      true
    }
  }
}
