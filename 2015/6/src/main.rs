fn main() {
  advent_of_code::run(first, second);
}

fn first(input: String) -> usize {
  let mut grid = [[false; 1000]; 1000];

  for instruction in parse_input(input) {
    let action: fn(&mut bool) = match instruction.action {
      Action::TurnOn => |x| *x = true,
      Action::TurnOff => |x| *x = false,
      Action::Toggle => |x| *x = !*x,
    };
    grid.iter_mut().take(instruction.right + 1).skip(instruction.left).for_each(|column| column.iter_mut().take(instruction.bottom + 1).skip(instruction.top).for_each(action));
  }

  grid.into_iter().map(|column| column.into_iter().filter(|x| *x).count()).sum()
}

fn second(input: String) -> u32 {
  let mut grid = [[0_u32; 1000]; 1000];

  for instruction in parse_input(input) {
    let action: fn(&mut u32) = match instruction.action {
      Action::TurnOn => |x| *x += 1,
      Action::TurnOff => |x| *x = x.checked_sub(1).unwrap_or(0),
      Action::Toggle => |x| *x += 2,
    };
    grid.iter_mut().take(instruction.right + 1).skip(instruction.left).for_each(|column| column.iter_mut().take(instruction.bottom + 1).skip(instruction.top).for_each(action));
  }

  grid.into_iter().map(|column| column.into_iter().sum::<u32>()).sum()
}

struct Instruction {
  action: Action,
  top: usize,
  right: usize,
  bottom: usize,
  left: usize,
}
enum Action {
  TurnOn,
  TurnOff,
  Toggle,
}

fn parse_input(input: String) -> Vec<Instruction> {
  input
    .lines()
    .map(|x| {
      let mut words_iter = x.split_whitespace();

      let action = match words_iter.next().unwrap() {
        "turn" => match words_iter.next().unwrap() {
          "on" => Action::TurnOn,
          "off" => Action::TurnOff,
          _ => panic!(),
        },
        "toggle" => Action::Toggle,
        _ => panic!(),
      };

      let first_coords = words_iter.next().unwrap().split_once(',').unwrap();
      let first_coords: (usize, usize) = (first_coords.0.parse().unwrap(), first_coords.1.parse().unwrap());
      words_iter.next();
      let second_coords = words_iter.next().unwrap().split_once(',').unwrap();
      let second_coords = (second_coords.0.parse().unwrap(), second_coords.1.parse().unwrap());

      let top = first_coords.1.min(second_coords.1);
      let right = first_coords.0.max(second_coords.0);
      let bottom = first_coords.1.max(second_coords.1);
      let left = first_coords.0.min(second_coords.0);

      Instruction { action, top, right, bottom, left }
    })
    .collect()
}
