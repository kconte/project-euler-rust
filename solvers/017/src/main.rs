#![warn(clippy::all)]
#![warn(clippy::pedantic)]

fn main() {
  run();
}

fn run() {
  let start = std::time::Instant::now();

  // code goes here
  let res: usize = (1..=1_000)
    .map(|i| i.to_words().replace("-", "").replace(" ", "").len())
    .sum();

  let span = start.elapsed().as_nanos();
  println!("{} {}", res, span);
}

#[inline]
fn div_rem(a: u64, b: u64) -> (u64, u64) {
  (a / b, a % b)
}

trait ToWords {
  fn to_words(&self) -> String;
}

impl ToWords for u64 {
  #[allow(clippy::cast_possible_truncation)]
  fn to_words(&self) -> String {
    let val = *self;

    if val >= 1_000 {
      let (div, rem) = div_rem(val, 1_000);
      if rem > 0 {
        format!("{} thousand {}", div.to_words(), rem.to_words())
      } else {
        format!("{} thousand", div.to_words())
      }
    } else if val >= 100 {
      let (div, rem) = div_rem(val, 100);
      if rem > 0 {
        format!("{} hundred and {}", div.to_words(), rem.to_words())
      } else {
        format!("{} hundred", div.to_words())
      }
    } else if val >= 20 {
      let (div, rem) = div_rem(val, 10);
      if rem > 0 {
        format!("{}-{}", TENS[div as usize], rem.to_words())
      } else {
        TENS[div as usize].to_string()
      }
    } else {
      DIGITS[val as usize].to_string()
    }
  }
}

const DIGITS: &[&str] = &[
  "zero",
  "one",
  "two",
  "three",
  "four",
  "five",
  "six",
  "seven",
  "eight",
  "nine",
  "ten",
  "eleven",
  "twelve",
  "thirteen",
  "fourteen",
  "fifteen",
  "sixteen",
  "seventeen",
  "eighteen",
  "nineteen",
];

const TENS: &[&str] = &[
  "", "ten", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
];
