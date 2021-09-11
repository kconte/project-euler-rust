#![warn(clippy::all)]
#![warn(clippy::pedantic)]

mod utils;

use clap::{App, Arg, SubCommand};

fn main() {
  // Determine what actions will be taken
  let _matches = App::new("Project Euler")
    .version("0.1.0")
    .author("Kevin Conte <kevin@conte.net>")
    .about("Runner and Benchmarker for Project Euler solutions")
    .subcommand(
      SubCommand::with_name("run")
        .about("runs a particular problem")
        .version("0.1.0")
        .author("Kevin Conte<kevin@conte.net>")
        .arg(
          Arg::with_name("all")
            .long("all")
            .short("a")
            .help("run all problem solvers")
            .conflicts_with("numbers"),
        )
        .arg(Arg::with_name("numbers").multiple(true)),
    )
    .subcommand(
      SubCommand::with_name("benchmark")
        .about("benchmarks a particular problem")
        .version("0.1.0")
        .author("Kevin Conte<kevin@conte.net>")
        .arg(
          Arg::with_name("all")
            .long("all")
            .short("a")
            .help("benchmark all problem solvers")
            .conflicts_with("numbers"),
        )
        .arg(Arg::with_name("numbers").multiple(true)),
    )
    .get_matches();
}
