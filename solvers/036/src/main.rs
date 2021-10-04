#![warn(clippy::all)]
#![warn(clippy::pedantic)]

fn main() {
  run();
}

fn run() {
  let start = std::time::Instant::now();

  // code goes here
  let res: u64 = (1..1_000_000)
    .filter(|&v| {
      is_palindrome(&to_string(v, 10)) && is_palindrome(&to_string(v, 2))
    })
    .sum();

  let span = start.elapsed().as_nanos();
  println!("{} {}", res, span);
}

fn is_palindrome(s: &str) -> bool {
  s == s.chars().rev().collect::<String>()
}

fn to_string(val: u64, radix: u64) -> String {
  let mut rep = String::new();
  let mut val = val;
  while val > 0 {
    let digit = val % radix;
    val /= radix;
    rep.push_str(&digit.to_string());
  }

  rep.chars().rev().collect()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn is_palindrome_works() {
    assert!(is_palindrome("585"));
    assert!(is_palindrome("1001001001"));
    assert!(!is_palindrome("1101001001"));
  }

  #[test]
  fn to_string_works() {
    assert_eq!(to_string(585, 10), "585");
    assert_eq!(to_string(586, 10), "586");
    assert_eq!(to_string(585, 2), "1001001001");
  }
}
