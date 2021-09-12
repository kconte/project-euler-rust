#![warn(clippy::all)]
#![warn(clippy::pedantic)]

fn main() {
  run();
}

fn run() {
  let start = std::time::Instant::now();

  // code goes here
  let mut limit: u64 = 600_851_475_143;
  let mut res = 2;

  while res * res <= limit {
    while limit % res == 0 {
      limit /= res;
    }
    res += 1;
  }

  if limit > res {
    res = limit;
  }

  let span = start.elapsed().as_nanos();
  println!("{} {}", res, span);
}
