use std::{
  env, fs,
  io::{self, Read},
  time::Instant,
};

use serde::Serialize;

pub fn run<F, T, S, U>(first: F, second: S)
where
  F: FnOnce(String) -> T,
  T: Serialize,
  S: FnOnce(String) -> U,
  U: Serialize,
{
  let mut input = String::new();
  io::stdin().read_to_string(&mut input).expect("stdin should contain valid UTF8");
  let args: Vec<_> = env::args().collect();

  let start = Instant::now();

  match &args[1][..] {
    "first" => {
      let result = first(input);
      write_result(&args[2], (start.elapsed(), result));
    }
    "second" => {
      let result = second(input);
      write_result(&args[2], (start.elapsed(), result));
    }
    _ => panic!(r#"the first arg should be either "first" or "second""#),
  };
}

fn write_result(file: &str, value: impl Serialize) {
  fs::write(file, serde_json::to_string(&value).expect("should be able to serialize output")).unwrap_or_else(|_| panic!("should be able to write to {file}"));
}
