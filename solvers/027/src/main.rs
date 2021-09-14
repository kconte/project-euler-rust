#![warn(clippy::all)]
#![warn(clippy::pedantic)]

fn main() {
  run();
}

#[allow(clippy::maybe_infinite_iter)]
fn run() {
  let start = std::time::Instant::now();

  // code goes here
  let mut res = 0;
  let mut max_primes = 0;

  for a in -999..=999 {
    for b in -1_000..=1_000 {
      let primes = (0..).take_while(|&n| is_prime(calc(n, a, b))).count();
      if primes > max_primes {
        max_primes = primes;
        res = a * b;
      }
    }
  }

  let span = start.elapsed().as_nanos();
  println!("{} {}", res, span);
}

#[inline]
fn calc(n: i64, a: i64, b: i64) -> i64 {
  n * n + a * n + b
}

fn is_prime(n: i64) -> bool {
  if n == 2 {
    return true;
  } else if n % 2 == 0 || n < 2 {
    return false;
  }

  let mut i = 3;
  while i * i <= n {
    if n % i == 0 {
      return false;
    }
    i += 2;
  }

  true
}
