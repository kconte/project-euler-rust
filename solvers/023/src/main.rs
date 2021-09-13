#![warn(clippy::all)]
#![warn(clippy::pedantic)]

fn main() {
  run();
}

#[allow(clippy::cast_possible_truncation)]
fn run() {
  let start = std::time::Instant::now();

  // code goes here
  let limit = 28123;

  let abundants: Vec<u64> = (1..=limit).filter(|&v| d(v) > v).collect();
  let mut abundant_sums = vec![false; (limit + 1) as usize];

  for i in 0..abundants.len() - 1 {
    for j in i..abundants.len() {
      let sum = abundants[i] + abundants[j];
      if sum <= limit {
        abundant_sums[sum as usize] = true;
      }
    }
  }

  let res = abundant_sums
    .into_iter()
    .enumerate()
    .filter(|(_, v)| !*v)
    .fold(0, |acc, (i, _)| acc + i) as u64;

  let span = start.elapsed().as_nanos();
  println!("{} {}", res, span);
}

fn d(n: u64) -> u64 {
  let mut limit = sqrt(n);
  let mut sum = 1;

  if limit * limit == n {
    limit -= 1;
    sum += limit;
  }

  for i in 2..=limit {
    if n % i == 0 {
      sum += i + (n / i);
    }
  }

  sum
}

#[allow(clippy::similar_names)]
fn sqrt(a: u64) -> u64 {
  let val = a;
  let mut min = 0;
  let mut max = a;

  while min < max {
    let mid = (min + max + 1) / 2;
    let mid_sq = mid * mid;
    match mid_sq.cmp(&val) {
      | std::cmp::Ordering::Equal => return mid,
      | std::cmp::Ordering::Less => min = mid,
      | std::cmp::Ordering::Greater => max = mid - 1,
    }
  }
  min
}
