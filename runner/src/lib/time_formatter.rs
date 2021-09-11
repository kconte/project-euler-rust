pub trait TimeFormatter {
  /// Converts an integer representing nanoseconds to a human-readable string.
  fn format_as_time(&self) -> String;
}

const MICROSECOND: u64 = 1_000;
const MILLISECOND: u64 = MICROSECOND * 1_000;
const SECOND: u64 = MILLISECOND * 1_000;
const MINUTE: u64 = SECOND * 60;

#[allow(clippy::cast_precision_loss)]
#[allow(clippy::non_ascii_literal)]
impl TimeFormatter for u64 {
  fn format_as_time(&self) -> String {
    // minutes
    if *self >= MINUTE {
      format!(
        "{} min {}",
        *self / MINUTE,                    // integer portion
        (*self % MINUTE).format_as_time()  // fractional portion
      )
    }
    // seconds
    else if *self >= SECOND {
      format!("{:.3} sec", (*self as f64) / SECOND as f64)
    }
    // milliseconds
    else if *self >= MILLISECOND {
      format!("{:.3} ms", (*self as f64) / MILLISECOND as f64)
    }
    // microseconds
    else if *self >= MICROSECOND {
      format!("{:.3} μs", (*self as f64) / MICROSECOND as f64)
    }
    // nanoseconds
    else {
      format!("{} ns", *self)
    }
  }
}

#[cfg(test)]
#[allow(clippy::non_ascii_literal)]
mod time_formatter_tests {
  use super::{TimeFormatter, MICROSECOND, MILLISECOND, MINUTE, SECOND};

  #[test]
  fn time_formatter() {
    // nanoseconds
    let mut span: u64 = 123;
    assert_eq!(span.format_as_time(), "123 ns");

    // microseconds
    span = MICROSECOND + 234;
    assert_eq!(span.format_as_time(), "1.234 μs");

    // milliseconds
    span = 7 * MILLISECOND + 654 * MICROSECOND;
    assert_eq!(span.format_as_time(), "7.654 ms");

    // seconds
    span = 9 * SECOND + 877 * MILLISECOND;
    assert_eq!(span.format_as_time(), "9.877 sec");

    // minutes
    span = 2 * MINUTE + 15 * SECOND + 543 * MILLISECOND;
    assert_eq!(span.format_as_time(), "2 min 15.543 sec");
  }
}
