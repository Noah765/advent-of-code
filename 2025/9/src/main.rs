fn main() {
  advent_of_code::run(first, second);
}

fn first(input: String) -> usize {
  let red_tiles = parse_red_tiles(&input);
  red_tiles.iter().flat_map(|a| red_tiles.iter().map(|b| (a.0.abs_diff(b.0) + 1) * (a.1.abs_diff(b.1) + 1))).max().unwrap()
}

fn second(input: String) -> usize {
  let red_tiles = parse_red_tiles(&input);

  let mut green_lines: Vec<_> = red_tiles.windows(2).map(|x| (x[0], x[1])).collect();
  green_lines.push((*red_tiles.last().unwrap(), *red_tiles.first().unwrap()));

  // We assume that the largest rectangle has a width and height larger than 1
  red_tiles
    .iter()
    .enumerate()
    .flat_map(|(i, a)| {
      let before_a = if i == 0 { *red_tiles.last().unwrap() } else { red_tiles[i - 1] };
      let after_a = if i == red_tiles.len() - 1 { *red_tiles.first().unwrap() } else { red_tiles[i + 1] };

      red_tiles
        .iter()
        .filter(move |b| {
          a.0 < b.0 && a.1 < b.1 && (a.0 < after_a.0 || a.1 < before_a.1 || a.0 > before_a.0 && a.1 > after_a.1) || a.0 > b.0 && a.1 < b.1 && (a.1 < after_a.1 || a.0 > before_a.0 || a.1 > before_a.1 && a.0 < after_a.0)
        })
        .filter(|b| {
          !green_lines.iter().any(|(start, end)| {
            start.0 == end.0 && start.0 > a.0.min(b.0) && end.0 < a.0.max(b.0) && (start.1 > a.1 || end.1 > a.1) && (start.1 < b.1 || end.1 < b.1)
              || start.1 == end.1 && start.1 > a.1 && end.1 < b.1 && (start.0 > a.0.min(b.0) || end.0 > a.0.min(b.0)) && (start.0 < a.0.max(b.0) || end.0 < a.0.max(b.0))
          })
        })
        .map(move |b| (a.0.abs_diff(b.0) + 1) * (a.1.abs_diff(b.1) + 1))
    })
    .max()
    .unwrap()
}

fn parse_red_tiles(input: &str) -> Vec<(usize, usize)> {
  input
    .lines()
    .map(|x| {
      let (x, y) = x.split_once(',').unwrap();
      (x.parse().unwrap(), y.parse().unwrap())
    })
    .collect()
}
