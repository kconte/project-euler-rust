#![warn(clippy::all)]
#![warn(clippy::pedantic)]

use num_integer::Roots;

fn main() {
  run();
}

fn run() {
  let start = std::time::Instant::now();

  // code goes here
  let mut res = 0;

  for i in 2..=10_000 {
    let a = d(i);
    if a > i && a < 10_000 {
      let b = d(a);
      if b == i {
        res += i + a;
      }
    }
  }

  let span = start.elapsed().as_nanos();
  println!("{} {}", res, span);
}

fn d(n: u64) -> u64 {
  let mut sqrt = n.sqrt();
  let mut res = 1;

  if sqrt * sqrt == res {
    res += sqrt;
    sqrt -= 1;
  }

  for i in 2..=sqrt {
    if n % i == 0 {
      res += i + (n / i);
    }
  }
  res
}
