use std::ops::IndexMut;

fn main() {
  advent_of_code::run(first, second);
}

fn first(input: String) -> u16 {
  parse_machines(&input).map(|machine| generate_button_subsets(&machine.buttons, machine.lights.len()).into_iter().filter(|(_, x)| (0..x.len()).all(|i| (x[i] % 2 == 1) == machine.lights[i])).map(|x| x.0).min().unwrap()).sum()
}

fn second(input: String) -> u16 {
  parse_machines(&input).map(|x| calculate_minimum_button_presses(&x.joltages, &generate_button_subsets(&x.buttons, x.joltages.len())).unwrap()).sum()
}

struct Machine {
  lights: Vec<bool>,
  buttons: Vec<Vec<usize>>,
  joltages: Vec<u16>,
}

fn parse_machines(input: &str) -> impl Iterator<Item = Machine> {
  input.lines().map(|x| {
    let (lights, rest) = x.split_once(' ').unwrap();
    let (buttons, joltages) = rest.rsplit_once(' ').unwrap();

    let lights = lights[1..lights.len() - 1].chars().map(|x| x == '#').collect();
    let buttons = buttons.split(' ').map(|x| x[1..x.len() - 1].split(',').map(|x| x.parse().unwrap()).collect()).collect();
    let joltages = joltages[1..joltages.len() - 1].split(',').map(|x| x.parse().unwrap()).collect();

    Machine { lights, buttons, joltages }
  })
}

fn generate_button_subsets(buttons: &[Vec<usize>], state_length: usize) -> Vec<(u16, Vec<u16>)> {
  if buttons.is_empty() {
    return vec![(0, vec![0; state_length])];
  }

  let mut subsets = generate_button_subsets(&buttons[1..], state_length);

  subsets.extend_from_within(..);
  subsets.index_mut(..subsets.len() / 2).into_iter().for_each(|(cost, presses)| {
    *cost += 1;
    buttons[0].iter().for_each(|&i| presses[i] += 1);
  });

  subsets
}

fn calculate_minimum_button_presses(joltages: &Vec<u16>, button_subsets: &Vec<(u16, Vec<u16>)>) -> Option<u16> {
  if joltages.iter().all(|&x| x == 0) {
    return Some(0);
  }

  button_subsets
    .iter()
    .filter(|(_, x)| (0..x.len()).all(|i| joltages[i] >= x[i] && x[i] % 2 == joltages[i] % 2))
    .filter_map(|(cost, presses)| {
      let state = (0..joltages.len()).map(|i| (joltages[i] - presses[i]) / 2).collect();
      calculate_minimum_button_presses(&state, button_subsets).map(|x| cost + 2 * x)
    })
    .min()
}
