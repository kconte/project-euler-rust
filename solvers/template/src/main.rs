#![warn(clippy::all)]
#![warn(clippy::pedantic)]

fn main() {
  run();
}

fn run() {
  let start = std::time::Instant::now();

  // code goes here
  let res = 0;

  let span = start.elapsed().as_nanos();
  println!("{} {}", res, span);
}
