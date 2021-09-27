#![warn(clippy::all)]
#![warn(clippy::pedantic)]

fn main() {
  run();
}

fn run() {
  let start = std::time::Instant::now();

  // code goes here

  let mut products: Vec<u64> = vec![];
  for m in 2..100 {
    let nbegin = if m > 9 { 123 } else { 1234 };
    let nend = 10_000 / m + 1;

    for n in nbegin..nend {
      let product = m * n;
      let concated = concat(concat(product, n), m);
      if (100_000_000..1_000_000_000).contains(&concated)
        && is_pandigital(concated)
        && !products.contains(&product)
      {
        products.push(product);
      }
    }
  }

  let res: u64 = products
    .iter()
    .sum();

  let span = start
    .elapsed()
    .as_nanos();
  println!("{} {}", res, span);
}

fn concat(a: u64, b: u64) -> u64 {
  let mut a = a;
  let mut c = b;

  while c > 0 {
    a *= 10;
    c /= 10;
  }

  a + b
}

fn is_pandigital(n: u64) -> bool {
  let mut digits: u64 = 0;
  let mut count: u64 = 0;
  let mut n = n;

  while n > 0 {
    let tmp = n;
    digits |= 1 << (n % 10 - 1);
    if tmp == digits {
      return false;
    }
    count += 1;
    n /= 10;
  }

  digits == (1 << count) - 1
}
