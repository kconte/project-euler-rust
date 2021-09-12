#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::non_ascii_literal)]

mod lib;

use clap::{App, Arg, SubCommand};
use lib::{SolverResult, TimeFormatter};
use pad::{Alignment, PadStr};
use std::{path::Path, process::Command};

fn main() {
  let matches = get_matches();

  let root = Path::new(matches.value_of("root").unwrap());
  let use_color = !matches.is_present("no-color");

  if let Some(matches) = matches.subcommand_matches("run") {
    let mut indexes = matches.values_of("numbers").unwrap_or_default().peekable();

    if matches.is_present("all") {
      print_header();
      // TODO: iterate all implemented solvers and run each of them
      print_footer();
    } else if indexes.peek().is_some() {
      print_header();
      for index in indexes {
        let name = format!("{:03}", index.parse::<u64>().unwrap());
        let path = root.join(name);
        let res = run_one(path);
        print_record(index.parse::<u64>().unwrap(), res, use_color);
      }
      print_footer();
    } else {
      eprintln!("Must provide at least one solver to run.");
      std::process::exit(1);
    }
  }
}

fn get_matches() -> clap::ArgMatches<'static> {
  App::new("Project Euler")
    .version("0.1.0")
    .author("Kevin Conte <kevin@conte.net>")
    .about("Runner and Benchmarker for Project Euler solutions")
    .arg(
      Arg::with_name("no-color")
        .long("no-color")
        .help("Don't print using colors"),
    )
    .arg(
      Arg::with_name("root")
        .long("root")
        .short("r")
        .help("Root directory from which to run solvers")
        .default_value("./solvers"),
    )
    .subcommand(
      SubCommand::with_name("run")
        .about("Runs a particular problem")
        .version("0.1.0")
        .author("Kevin Conte<kevin@conte.net>")
        .arg(
          Arg::with_name("all")
            .long("all")
            .short("a")
            .help("Run all problem solvers")
            .conflicts_with("numbers"),
        )
        .arg(Arg::with_name("numbers").multiple(true)),
    )
    .subcommand(
      SubCommand::with_name("benchmark")
        .about("Benchmarks a particular problem")
        .version("0.1.0")
        .author("Kevin Conte<kevin@conte.net>")
        .arg(
          Arg::with_name("all")
            .long("all")
            .short("a")
            .help("Benchmark all problem solvers")
            .conflicts_with("numbers"),
        )
        .arg(
          Arg::with_name("iterations")
            .long("iterations")
            .short("i")
            .help("Number of samples to take")
            .default_value("1000"),
        )
        .arg(Arg::with_name("numbers").multiple(true)),
    )
    .get_matches()
}

fn run_one<P: AsRef<Path>>(idx: P) -> SolverResult {
  let mut cmd = Command::new("cargo");
  cmd.arg("run");
  cmd.arg("--quiet");
  cmd.arg("--release");
  cmd.current_dir(idx.as_ref());

  SolverResult::from(cmd.output().unwrap().stdout)
}

/// Debug utility that prints the type of a value.
///
/// # Examples
/// Basic usage:
/// ```rust
/// let a: u64 = 100;
/// print_type(&a);
/// // u64
///
/// let b: Vec<u8> = vec![]
/// print_type(&b);
/// // alloc::vec::Vec<u8>
/// ```
pub fn print_type<T>(_: &T) {
  println!("{}", std::any::type_name::<T>());
}

fn print_header() {
  println!("┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓");
  println!(
    "{}┃              Project Euler              ┃{}",
    termion::style::Bold,
    termion::style::Reset
  );
  println!("┣━━━━━┳━━━━━━━━━━━━━━━━━┳━━━━━━━━━━━━━━━━━┫");
  println!(
    "{}┃ idx ┃      result     ┃     runtime     ┃{}",
    termion::style::Bold,
    termion::style::Reset
  );
  println!("┡━━━━━╇━━━━━━━━━━━━━━━━━╇━━━━━━━━━━━━━━━━━┩");
}

fn print_footer() {
  println!("└─────┴─────────────────┴─────────────────┘");
}

fn print_record(idx: u64, res: SolverResult, use_color: bool) {
  let idx_str = idx
    .to_string()
    .pad_to_width_with_alignment(3, Alignment::Right);
  let res_str = res.result.to_string().pad_to_width(15);
  let tim_str = res.time_taken.format_as_time().pad_to_width(15);

  if use_color {
    println!(
      "│ {}{}{} │ {}{}{} │ {}{}{} │",
      termion::color::Fg(termion::color::Magenta),
      idx_str,
      termion::color::Fg(termion::color::Reset),
      termion::color::Fg(termion::color::Green),
      res_str,
      termion::color::Fg(termion::color::Reset),
      termion::color::Fg(termion::color::Cyan),
      tim_str,
      termion::color::Fg(termion::color::Reset),
    );
  } else {
  }
}
