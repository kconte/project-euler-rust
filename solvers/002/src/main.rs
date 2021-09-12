#![warn(clippy::all)]
#![warn(clippy::pedantic)]

fn main() {
  run();
}

fn run() {
  let start = std::time::Instant::now();

  // code goes here
  let res: u64 = Fibonacci::default()
    .take_while(|f| *f < 4_000_000)
    .filter(|v| v & 1 == 0)
    .sum();

  let span = start.elapsed().as_nanos();
  println!("{} {}", res, span);
}

struct Fibonacci {
  a: u64,
  b: u64,
}

impl Default for Fibonacci {
  fn default() -> Self {
    Self {
      a: 0, b: 1
    }
  }
}

impl Iterator for Fibonacci {
  type Item = u64;

  fn next(&mut self) -> Option<Self::Item> {
    let c = self.a + self.b;
    self.a = self.b;
    self.b = c;
    Some(self.a)
  }
}
