#![warn(clippy::all)]
#![warn(clippy::pedantic)]

fn main() {
  run();
}

fn run() {
  let start = std::time::Instant::now();

  // code goes here
  let mut res = 0;

  for a in 100..1000 {
    for b in 100..1000 {
      let c = a * b;
      if is_palindrome(c) && c > res {
        res = c;
      }
    }
  }

  let span = start.elapsed().as_nanos();
  println!("{} {}", res, span);
}

fn is_palindrome(n: u64) -> bool {
  let copy = n;
  let mut n = n;
  let mut rev = 0;

  while n > 0 {
    rev *= 10;
    rev += n % 10;
    n /= 10;
  }

  rev == copy
}
