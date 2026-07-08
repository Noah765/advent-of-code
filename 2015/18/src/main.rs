fn main() {
    advent_of_code::run(first, second);
}

fn first(input: String) -> usize {
    run::<false>(&input)
}

fn second(input: String) -> usize {
    run::<true>(&input)
}

fn run<const CORNERS: bool>(input: &str) -> usize {
    let mut grid = [[false; 100]; 100];
    for (i, line) in input.lines().enumerate() {
        for (j, _) in line.bytes().enumerate().filter(|x| x.1 == b'#') {
            grid[i][j] = true;
        }
    }

    (0..100).for_each(|_| grid = step::<CORNERS>(grid));

    grid.iter().map(|x| x.iter().filter(|&&x| x).count()).sum()
}

fn step<const CORNERS: bool>(prev: [[bool; 100]; 100]) -> [[bool; 100]; 100] {
    let mut next = [[false; 100]; 100];

    for i in 0usize..100 {
        for j in 0usize..100 {
            let count = (i.saturating_sub(1)..=i + 1)
                .flat_map(|row| (j.saturating_sub(1)..=j + 1).map(move |x| (row, x)))
                .filter(|&x| x != (i, j))
                .filter(|x| prev.get(x.0).and_then(|row| row.get(x.1)) == Some(&true))
                .count();
            next[i][j] = count == 3 || prev[i][j] && count == 2;
        }
    }

    if CORNERS {
        next[0][0] = true;
        next[0][99] = true;
        next[99][0] = true;
        next[99][99] = true;
    }

    next
}
