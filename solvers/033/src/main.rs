#![warn(clippy::all)]
#![warn(clippy::pedantic)]

fn main() {
  run();
}

fn run() {
  let start = std::time::Instant::now();

  // code goes here
  let mut num = 1;
  let mut den = 1;

  for n in 10..100 {
    for d in (n + 1)..100 {
      if is_non_trivial(n, d) {
        num *= n;
        den *= d;
      }
    }
  }

  let g = gcd(num, den);
  let res = den / g;

  let span = start
    .elapsed()
    .as_nanos();
  println!("{} {}", res, span);
}

#[allow(clippy::cast_lossless)]
#[allow(clippy::cast_precision_loss)]
#[allow(clippy::float_cmp)]
fn is_non_trivial(n: u64, d: u64) -> bool {
  if n % 10 == 0 && d % 10 == 0 {
    return false;
  }

  let n_t = n / 10;
  let n_o = n % 10;
  let d_t = d / 10;
  let d_o = d % 10;

  let f_1 = (n_t as f64) / (d_o as f64);
  let f_2 = (n_o as f64) / (d_t as f64);
  let f_o = (n as f64) / (d as f64);

  if n_t == d_o {
    f_2 == f_o
  } else if n_o == d_t {
    f_1 == f_o
  } else {
    false
  }
}

fn gcd(a: u64, b: u64) -> u64 {
  match a.cmp(&b) {
    | std::cmp::Ordering::Equal => a,
    | std::cmp::Ordering::Greater => gcd(a - b, b),
    | std::cmp::Ordering::Less => gcd(a, b - a),
  }
}
