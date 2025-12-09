fn main() {
  advent_of_code::run(first, second);
}

fn first(input: String) -> usize {
  let boxes = parse_boxes(&input);
  let connection_limit = if boxes.len() == 20 { 10 } else { 1000 };

  let mut circuits: Vec<_> = (0..boxes.len()).map(|x| vec![x]).collect();
  construct_sorted_box_pairs(&boxes).into_iter().take(connection_limit).for_each(|(i, j)| update_circuits(i, j, &mut circuits));

  let mut circuit_sizes: Vec<_> = circuits.into_iter().map(|x| x.len()).collect();
  circuit_sizes.sort_unstable();
  circuit_sizes.into_iter().rev().take(3).product()
}

fn second(input: String) -> i64 {
  let boxes = parse_boxes(&input);

  let mut box_pairs = construct_sorted_box_pairs(&boxes).into_iter();
  let mut circuits: Vec<_> = (0..boxes.len()).map(|x| vec![x]).collect();
  let (i, j) = loop {
    let (i, j) = box_pairs.next().unwrap();
    update_circuits(i, j, &mut circuits);
    if circuits.len() == 1 {
      break (i, j);
    }
  };

  boxes[i].0 * boxes[j].0
}

fn parse_boxes(input: &str) -> Vec<(i64, i64, i64)> {
  input
    .lines()
    .map(|x| {
      let mut coordinates = x.splitn(3, ',');
      (coordinates.next().unwrap().parse().unwrap(), coordinates.next().unwrap().parse().unwrap(), coordinates.next().unwrap().parse().unwrap())
    })
    .collect()
}

fn construct_sorted_box_pairs(boxes: &Vec<(i64, i64, i64)>) -> Vec<(usize, usize)> {
  let mut pairs: Vec<_> = (0..boxes.len()).flat_map(|i| (i + 1..boxes.len()).map(move |j| (i, j))).collect();
  pairs.sort_unstable_by_key(|&(i, j)| (boxes[i].0 - boxes[j].0).pow(2) + (boxes[i].1 - boxes[j].1).pow(2) + (boxes[i].2 - boxes[j].2).pow(2));
  pairs
}

fn update_circuits(first_box_index: usize, second_box_index: usize, circuits: &mut Vec<Vec<usize>>) {
  let first_circuit_index = circuits.iter().position(|x| x.contains(&first_box_index)).unwrap();
  let second_circuit_index = circuits.iter().position(|x| x.contains(&second_box_index)).unwrap();

  if first_circuit_index == second_circuit_index {
    return;
  }

  let mut second_circuit = circuits.swap_remove(second_circuit_index);
  let first_circuit_index = if first_circuit_index < circuits.len() { first_circuit_index } else { second_circuit_index };
  circuits[first_circuit_index].append(&mut second_circuit);
}
