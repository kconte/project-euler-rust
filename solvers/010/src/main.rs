#![warn(clippy::all)]
#![warn(clippy::pedantic)]

fn main() {
  run();
}

fn run() {
  let start = std::time::Instant::now();

  // code goes here
  let res: u64 = primes_under(2_000_000).iter().sum();

  let span = start.elapsed().as_nanos();
  println!("{} {}", res, span);
}

/// iterative implementation of Sieve of Eratosthenes
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_sign_loss)]
#[allow(clippy::cast_precision_loss)]
fn primes_under(n: u64) -> Vec<u64> {
  let mut is_prime = vec![true; (n + 1) as usize];
  is_prime[0] = false;
  is_prime[1] = false;

  let sqrt = (n as f64).sqrt() as usize;
  for i in 2..=sqrt {
    if is_prime[i] {
      for j in (i * i..=n as usize).step_by(i) {
        is_prime[j] = false;
      }
    }
  }

  is_prime
    .into_iter()
    .enumerate()
    .filter(|(_, v)| *v)
    .map(|(i, _)| i as u64)
    .collect()
}
