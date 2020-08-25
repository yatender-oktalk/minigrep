use std::error::Error;
use std::fs;

pub struct Config {
  pub query: String,
  pub filename: String,
}

impl Config {
  pub fn new(args: &[String]) -> Result<Config, &'static str> {
    if args.len() < 3 {
      return Err("Not enough arguments");
    }

    let query = args[1].clone();
    let filename = args[2].clone();

    Ok(Config { query, filename })
  }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  let contents = fs::read_to_string(config.filename).expect("Something went wrong with the file");

  for line in search(&config.query, &contents) {
    println!("{}", line);
  }
  Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  let mut results = Vec::new();
  for line in contents.lines() {
    if line.contains(query) {
      results.push(line.trim());
    }
  }
  println!("{:?}", results);
  results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  let mut results = Vec::new();
  let query = query.to_lowercase();

  for line in contents.lines() {
    if line.to_lowercase().contains(&query) {
      results.push(line.trim());
    }
  }

  println!("{:?}", results);
  results
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn one_result() {
    let query = "duct";
    let contents = "\
    Rust:
    safe, fast, productive.
    pick three.";

    assert_eq!(vec!["safe, fast, productive."], search(query, contents));
  }

  #[test]
  fn case_sensitive_search() {
    let query = "duct";
    let contents = "\
      Rust:
      safe, fast, productive.
      Pick three.
      Duct tape.";

    assert_eq!(vec!["safe, fast, productive."], search(query, contents));
  }
  #[test]
  fn case_insensitive_search() {
    let query = "Productive";
    let contents = "\
    Rust:
    safe, fast, productive.
    Pick three.
    Duct tape.";

    assert_eq!(
      vec!["safe, fast, productive."],
      search_case_insensitive(query, contents)
    );
  }
}
