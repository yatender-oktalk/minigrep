use std::env;
use std::fs;

fn main() {
  let args: Vec<String> = env::args().collect();

  let config = Config::new(&args);
}

struct Config {
  query: String,
  filename: String,
}

impl Config {
  fn new(args: &[String]) -> Config {
    if args.len() < 3 {
      panic!("Not enough arguments");
    }

    let query = args[1].clone();
    let filename = args[2].clone();

    Config { query, filename }
  }
}
