#![warn(clippy::all)]
#![warn(clippy::pedantic)]

fn main() {
  run();
}

fn run() {
  let start = std::time::Instant::now();

  // code goes here
  let res: u64 = (3..=2_177_280)
    .filter(|&i| {
      digits(i)
        .iter()
        .map(|&i| factorial(i))
        .sum::<u64>()
        == i
    })
    .sum();

  let span = start
    .elapsed()
    .as_nanos();
  println!("{} {}", res, span);
}

fn factorial(n: u64) -> u64 {
  match n {
    | 0 => 1,
    | _ => (1..=n).product(),
  }
}

fn digits(n: u64) -> Vec<u64> {
  let mut n = n;
  let mut dig = vec![];

  while n > 0 {
    dig.push(n % 10);
    n /= 10;
  }

  dig
}
