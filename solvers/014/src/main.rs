#![warn(clippy::all)]
#![warn(clippy::pedantic)]

use std::collections::HashMap;

fn main() {
  run();
}

fn run() {
  let start = std::time::Instant::now();

  // code goes here
  let mut collatz = Collatz::default();
  let res = (1..1_000_000)
    .enumerate()
    .map(|(i, v)| (i, collatz.find(v)))
    .max_by_key(|p| p.1)
    .unwrap()
    .0 as u64
    + 1;

  let span = start.elapsed().as_nanos();
  println!("{} {}", res, span);
}

struct Collatz {
  memo: HashMap<u64, u64>,
}

impl Default for Collatz {
  fn default() -> Self {
    let mut hm = HashMap::new();
    hm.insert(1, 1);
    Self {
      memo: hm
    }
  }
}

impl Collatz {
  fn find(&mut self, key: u64) -> u64 {
    if self.memo.contains_key(&key) {
      return *self.memo.get(&key).unwrap();
    }

    let next = if key % 2 == 0 { key / 2 } else { 3 * key + 1 };

    let val = 1 + self.find(next);
    self.memo.insert(key, val);
    val
  }
}
