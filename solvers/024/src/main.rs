#![warn(clippy::all)]
#![warn(clippy::pedantic)]

use factorial::Factorial;

fn main() {
  run();
}

#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::similar_names)]
fn run() {
  let start = std::time::Instant::now();

  // code goes here
  let digits = nth_lexicographic_permutation(999_999, &mut vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);

  // convert digits to integer
  let mut res = 0;
  for digit in digits {
    res *= 10;
    res += digit;
  }

  let span = start.elapsed().as_nanos();
  println!("{} {}", res, span);
}

#[allow(clippy::similar_names)]
#[allow(clippy::cast_possible_truncation)]
fn nth_lexicographic_permutation(n: u64, set: &mut Vec<u64>) -> Vec<u64> {
  let mut res = vec![];
  let mut idx = n;

  while !set.is_empty() {
    let perms = (set.len() - 1).factorial() as u64;
    let ridx = idx / perms;
    idx %= perms;
    res.push(set.remove(ridx as usize));
  }

  res
}

#[cfg(test)]
mod tests {
  use super::nth_lexicographic_permutation;

  #[test]
  fn it_works() {
    assert_eq!(
      vec![0, 1, 2],
      nth_lexicographic_permutation(0, &mut vec![0, 1, 2])
    );

    assert_eq!(
      vec![0, 2, 1],
      nth_lexicographic_permutation(1, &mut vec![0, 1, 2])
    );

    assert_eq!(
      vec![1, 0, 2],
      nth_lexicographic_permutation(2, &mut vec![0, 1, 2])
    );

    assert_eq!(
      vec![1, 2, 0],
      nth_lexicographic_permutation(3, &mut vec![0, 1, 2])
    );

    assert_eq!(
      vec![2, 0, 1],
      nth_lexicographic_permutation(4, &mut vec![0, 1, 2])
    );

    assert_eq!(
      vec![2, 1, 0],
      nth_lexicographic_permutation(5, &mut vec![0, 1, 2])
    );
  }
}
