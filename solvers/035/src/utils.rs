use num_traits::Zero;

/// Algorithm: [`Wikipedia`]<https://en.wikipedia.org/wiki/Integer_square_root#Example_implementation_in_C>.
/// Calculates the integer square root of an integer.
pub trait Sqrt {
  fn sqrt(&self) -> Self;
}

impl Sqrt for u64 {
  fn sqrt(&self) -> Self {
    let val = *self;
    if val == 1 {
      return *self;
    }

    let mut x0 = val >> 1;
    let mut x1 = (x0 + val / x0) >> 1;
    while x1 < x0 {
      x0 = x1;
      x1 = (x0 + val / x0) >> 1;
    }

    x0
  }
}

/// Returns a vector of all primes under a given limit. Uses the Sieve of
/// Eratosthenes algorithm.
#[allow(clippy::cast_possible_truncation)]
pub fn sieve(n: u64) -> Vec<u64> {
  let mut is_prime = vec![true; (n + 1) as usize];
  is_prime[0] = false;
  is_prime[1] = false;

  for i in 2..=n.sqrt() as usize {
    if is_prime[i] {
      for j in (i * i..=n as usize).step_by(i) {
        is_prime[j] = false;
      }
    }
  }

  is_prime
    .into_iter()
    .enumerate()
    .filter_map(|(i, v)| if v { Some(i as u64) } else { None })
    .collect()
}

/// Converts an integer to a vector representation of its digits
pub trait ToDigits: num_integer::Integer + Clone {
  fn to_digits(&self, radix: Self) -> Vec<Self> {
    let mut val = self.clone();
    let mut digits = vec![];

    while !val.is_zero() {
      digits.push(val.clone() % radix.clone());
      val = val.clone() / radix.clone();
    }

    digits
  }
}
impl ToDigits for u64 {}

/// Converts a vector representation of an integer's digits to an integer
pub trait FromDigits: num_integer::Integer + Clone {
  fn from_digits(digits: &mut Vec<Self>, radix: Self) -> Self {
    let mut val: Self = Zero::zero();
    while let Some(digit) = digits.pop() {
      val = val.clone() * radix.clone();
      val = val.clone() + digit.clone();
    }
    val
  }
}
impl FromDigits for u64 {}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn sieve_works() {
    assert_eq!(sieve(10), vec![2, 3, 5, 7]);
  }

  #[test]
  fn sqrt_works() {
    assert_eq!(100_u64.sqrt(), 10);
    assert_eq!(17_u64.sqrt(), 4);
  }

  #[test]
  fn to_digits_works() {
    assert_eq!(100.to_digits(10), vec![0, 0, 1]);
    assert_eq!(21_536.to_digits(10), vec![6, 3, 5, 1, 2]);
  }

  #[test]
  fn from_digits_works() {
    assert_eq!(u64::from_digits(&mut vec![0, 0, 1], 10), 100);
    assert_eq!(u64::from_digits(&mut vec![6, 3, 5, 1, 2], 10), 21_536);
  }
}
