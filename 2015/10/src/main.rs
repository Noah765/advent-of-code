fn main() {
  advent_of_code::run(first, second);
}

fn first(input: String) -> usize {
  (0..40).fold(input, |acc, _| next(acc)).len()
}

fn second(input: String) -> usize {
  (0..50).fold(input, |acc, _| next(acc)).len()
}

fn next(input: String) -> String {
  let mut result = String::new();

  let mut count = 0;
  let mut previous_char = input.chars().next().unwrap();
  for x in input.chars() {
    if x == previous_char {
      count += 1;
      continue;
    }
    result.push_str(&count.to_string());
    result.push(previous_char);
    count = 1;
    previous_char = x;
  }
  result.push_str(&count.to_string());
  result.push(previous_char);

  result
}
