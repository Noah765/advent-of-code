use std::{
    env, fs,
    io::{self, Read},
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
    io::stdin()
        .read_to_string(&mut input)
        .expect("stdin should contain valid UTF8");
    let args: Vec<_> = env::args().collect();
    match &args[1][..] {
        "first" => write_result(&args[2], first(input)),
        "second" => write_result(&args[2], second(input)),
        _ => panic!(r#"the first arg should be either "first" or "second""#),
    };
}

fn write_result(file: &str, value: impl Serialize) {
    fs::write(
        file,
        serde_json::to_string(&value).expect("should be able to serialize output"),
    )
    .unwrap_or_else(|_| panic!("should be able to write to {file}"));
}
