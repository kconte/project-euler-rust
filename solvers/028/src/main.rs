#![warn(clippy::all)]
#![warn(clippy::pedantic)]

fn main() {
  run();
}

fn run() {
  let start = std::time::Instant::now();

  // code goes here
  let res = sum_diagonals(500);

  let span = start.elapsed().as_nanos();
  println!("{} {}", res, span);
}

fn sum_diagonals(rows: u64) -> u64 {
  (1..=rows)
    .map(|n| {
      let mut sq = 2 * n + 1;
      sq *= sq;

      4 * sq - 2 * n - 4 * n - 6 * n
    })
    .sum::<u64>()
    + 1
}
