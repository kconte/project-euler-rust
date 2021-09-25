#![warn(clippy::all)]
#![warn(clippy::pedantic)]

fn main() {
  run();
}

fn run() {
  let start = std::time::Instant::now();

  // code goes here
  let mut res = 0;

  for i in 2..300_000 {
    let digit_fifth_powers: u64 = to_digits(i)
      .iter()
      .map(|&i| pow(i, 5))
      .sum();

    if digit_fifth_powers == i {
      res += digit_fifth_powers;
    }
  }

  let span = start
    .elapsed()
    .as_nanos();
  println!("{} {}", res, span);
}

fn to_digits(n: u64) -> Vec<u64> {
  let mut digits = vec![];
  let mut n = n;

  while n > 0 {
    digits.push(n % 10);
    n /= 10;
  }

  digits
}

#[inline]
fn pow(b: u64, e: u64) -> u64 {
  (0..e)
    .map(|_| b)
    .product()
}
