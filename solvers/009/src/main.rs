#![warn(clippy::all)]
#![warn(clippy::pedantic)]

fn main() {
  run();
}

fn run() {
  let start = std::time::Instant::now();

  // code goes here
  let res = find();

  let span = start.elapsed().as_nanos();
  println!("{} {}", res, span);
}

fn find() -> u64 {
  for a in 1..1000 {
    for b in 1..1000 {
      let c = 1000 - a - b;
      if a * a + b * b == c * c {
        return a * b * c;
      }
    }
  }

  unreachable!()
}
