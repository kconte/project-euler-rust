#![warn(clippy::all)]
#![warn(clippy::pedantic)]

fn main() {
  run();
}

fn run() {
  let start = std::time::Instant::now();

  // code goes here
  let res = (1..1_000).max_by_key(|n| cycle_len(*n)).unwrap();

  let span = start.elapsed().as_nanos();
  println!("{} {}", res, span);
}

#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::match_on_vec_items)]
fn cycle_len(n: u64) -> u64 {
  // early return condition
  if n == 1 {
    return 1;
  }

  let mut buffer = vec![None; n as usize];
  let mut remainder = 1;
  let mut idx = 1_u64;
  loop {
    let new_remainder = remainder % n;
    match buffer[new_remainder as usize] {
      | Some(i) => return idx - i,
      | None => buffer[new_remainder as usize] = Some(idx),
    }
    idx += 1;
    remainder = new_remainder * 10;
  }
}
