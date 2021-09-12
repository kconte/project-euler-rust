#![warn(clippy::all)]
#![warn(clippy::pedantic)]

fn main() {
  run();
}

fn run() {
  let start = std::time::Instant::now();

  // code goes here
  let mut counts = [0; 7];
  let mut day = 1; // 1 jan 1900 is a monday
  day = (day + days_in_year(1900)) % 7;
  for yr in 1901..=2000 {
    day = add_days(yr, day, &mut counts);
  }

  let res = counts[0];

  let span = start.elapsed().as_nanos();
  println!("{} {}", res, span);
}

fn is_leap_year(yr: u64) -> bool {
  if yr % 400 == 0 {
    true
  } else if yr % 100 == 0 {
    false
  } else {
    yr % 4 == 0
  }
}

#[inline]
fn days_in_year(yr: u64) -> u64 {
  if is_leap_year(yr) {
    366
  } else {
    365
  }
}

fn days_in_each_month(yr: u64) -> [u64; 12] {
  [
    31,                                     // jan
    if is_leap_year(yr) { 29 } else { 28 }, // feb
    31,                                     // mar
    30,                                     // apr
    31,                                     // may
    30,                                     // jun
    31,                                     // jul
    31,                                     // aug
    30,                                     // sep
    31,                                     // oct
    30,                                     // nov
    31,                                     // dec
  ]
}

#[allow(clippy::cast_possible_truncation)]
fn add_days(yr: u64, day: u64, counts: &mut [u64; 7]) -> u64 {
  let mut day = day;
  let dim = days_in_each_month(yr);
  for d in &dim {
    counts[day as usize] += 1;
    day = (day + *d) % 7;
  }
  day
}
