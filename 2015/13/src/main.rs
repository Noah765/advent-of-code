fn main() {
    advent_of_code::run(first, second);
}

fn first(input: String) -> i16 {
    find_optimal_seating_arrangement(parse_input(input))
}

fn second(input: String) -> i16 {
    let mut matrix = parse_input(input);
    matrix.iter_mut().for_each(|x| x.push(0));
    matrix.push(vec![0; matrix.len() + 1]);

    find_optimal_seating_arrangement(matrix)
}

fn parse_input(input: String) -> Vec<Vec<i16>> {
    let first_name = input.lines().next().unwrap().split(' ').next().unwrap();
    let n = input
        .lines()
        .position(|x| x.split(' ').next().unwrap() != first_name)
        .unwrap()
        + 1;

    let mut matrix = vec![Vec::with_capacity(n); n];
    let mut lines = input.lines();

    #[allow(clippy::needless_range_loop)]
    for i in 0..n {
        for j in 0..n {
            if i == j {
                matrix[i].push(0);
                continue;
            }

            let mut words = lines.next().unwrap().split(' ');
            let is_plus = words.nth(2).unwrap() == "gain";
            let factor = if is_plus { 1 } else { -1 };
            let mut value = words.next().unwrap().parse::<i16>().unwrap() * factor;

            if j < i {
                matrix[j][i] += value;
                value = matrix[j][i];
            }
            matrix[i].push(value);
        }
    }

    matrix
}

fn find_optimal_seating_arrangement(matrix: Vec<Vec<i16>>) -> i16 {
    fn explore(matrix: &Vec<Vec<i16>>, visited: &mut [bool], prev: usize) -> i16 {
        let mut i = 0;
        let mut max = i16::MIN;

        while i < visited.len() {
            if !visited[i] {
                visited[i] = true;
                max = max.max(matrix[prev][i] + explore(matrix, visited, i));
                visited[i] = false;
            }
            i += 1;
        }

        if max == i16::MIN {
            matrix[prev][0]
        } else {
            max
        }
    }

    let mut visited = vec![false; matrix.len()];
    visited[0] = true;
    explore(&matrix, &mut visited, 0)
}
