use std::fmt;

use super::TimeFormatter;
use num_bigint::BigUint;

#[derive(Debug)]
pub struct SolverResult {
  pub result:     BigUint,
  pub time_taken: u64,
}

impl fmt::Display for SolverResult {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}    {}", self.result, self.time_taken.format_as_time())
  }
}

impl From<String> for SolverResult {
  fn from(s: String) -> Self {
    let parts = s
      .split_ascii_whitespace()
      .map(std::string::ToString::to_string)
      .collect::<Vec<String>>();

    assert_eq!(parts.len(), 2);

    let result = parts[0]
      .parse::<BigUint>()
      .expect("could not parse BigUint");
    let time_taken = parts[1].parse::<u64>().expect("could not parse u64");

    Self {
      result,
      time_taken,
    }
  }
}

impl From<&str> for SolverResult {
  fn from(s: &str) -> Self {
    SolverResult::from(String::from(s))
  }
}

impl From<Vec<u8>> for SolverResult {
  fn from(s: Vec<u8>) -> Self {
    SolverResult::from(String::from_utf8_lossy(s.as_slice()).to_string())
  }
}
