fn main() {
  advent_of_code::run(first, second);
}

fn first(input: String) -> usize {
  get_accessible_paper_positions(&construct_grid(&input)).len()
}

fn second(input: String) -> usize {
  let mut grid = construct_grid(&input);

  let mut result = 0;

  let mut accessible_paper_positions = get_accessible_paper_positions(&grid);
  while !accessible_paper_positions.is_empty() {
    result += accessible_paper_positions.len();

    for (x, y) in accessible_paper_positions {
      grid[y][x] = Cell::Empty;
    }

    accessible_paper_positions = get_accessible_paper_positions(&grid);
  }

  result
}

#[derive(PartialEq)]
enum Cell {
  Empty,
  Paper,
}

fn construct_grid(input: &str) -> Vec<Vec<Cell>> {
  input.lines().map(|x| x.chars().map(|x| if x == '@' { Cell::Paper } else { Cell::Empty }).collect()).collect()
}

fn get_accessible_paper_positions(grid: &Vec<Vec<Cell>>) -> Vec<(usize, usize)> {
  grid.iter().enumerate().map(|(y, row)| row.iter().enumerate().filter(move |(x, cell)| **cell == Cell::Paper && count_surrounding_papers(&grid, *x, y) < 4).map(move |(x, _)| (x, y))).flatten().collect()
}

fn count_surrounding_papers(grid: &Vec<Vec<Cell>>, x: usize, y: usize) -> usize {
  let surrounding = [
    grid.get(y - 1).and_then(|row| row.get(x - 1)),
    grid.get(y - 1).map(|row| &row[x]),
    grid.get(y - 1).and_then(|row| row.get(x + 1)),
    grid[y].get(x + 1),
    grid[y].get(x - 1),
    grid.get(y + 1).and_then(|row| row.get(x - 1)),
    grid.get(y + 1).map(|row| &row[x]),
    grid.get(y + 1).and_then(|row| row.get(x + 1)),
  ];
  surrounding.iter().filter(|x| x.is_some_and(|x| *x == Cell::Paper)).count()
}
