use std::collections::{HashMap, HashSet};

fn main() {
  advent_of_code::run(first, second);
}

fn first(input: String) -> u16 {
  let distances = parse_input(&input);
  distances.keys().map(|start| visit(true, start, &mut HashSet::new(), &distances)).min().unwrap()
}

fn second(input: String) -> u16 {
  let distances = parse_input(&input);
  distances.keys().map(|start| visit(false, start, &mut HashSet::new(), &distances)).max().unwrap()
}

fn visit<'a>(shortest: bool, location: &'a str, visited: &mut HashSet<&'a str>, distances: &'a HashMap<&str, Vec<(&str, u16)>>) -> u16 {
  visited.insert(location);
  let iter = distances.get(location).unwrap().iter().filter_map(|(location, distance)| if visited.contains(location) { None } else { Some(distance + visit(shortest, location, visited, distances)) });
  let result = if shortest { iter.min().unwrap_or(0) } else { iter.max().unwrap_or(0) };
  visited.remove(location);
  result
}

fn parse_input(input: &str) -> HashMap<&str, Vec<(&str, u16)>> {
  let mut distances = HashMap::<_, Vec<_>>::new();

  for line in input.lines() {
    let mut words_iter = line.split_whitespace();
    let location1 = words_iter.next().unwrap();
    let location2 = words_iter.nth(1).unwrap();
    let distance = words_iter.nth(1).unwrap().parse().unwrap();
    distances.entry(location1).or_default().push((location2, distance));
    distances.entry(location2).or_default().push((location1, distance));
  }

  distances
}
