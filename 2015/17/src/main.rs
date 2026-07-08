fn main() {
    advent_of_code::run(first, second);
}

fn first(input: String) -> usize {
    let containers: Vec<u8> = input.lines().map(|x| x.parse().unwrap()).collect();
    let mut result = Vec::new();
    collect_combinations(&mut result, &containers, 0, 0, 0);
    result.len()
}

fn second(input: String) -> usize {
    let containers: Vec<u8> = input.lines().map(|x| x.parse().unwrap()).collect();

    let mut result = Vec::new();
    collect_combinations(&mut result, &containers, 0, 0, 0);

    let minimum = *result.iter().min().unwrap();
    result.iter().filter(|&&x| x == minimum).count()
}

fn collect_combinations(result: &mut Vec<u8>, containers: &[u8], i: usize, sum: u8, count: u8) {
    if sum == 150 {
        result.push(count);
    } else if sum < 150 && i < containers.len() {
        collect_combinations(result, containers, i + 1, sum, count);
        collect_combinations(result, containers, i + 1, sum + containers[i], count + 1);
    }
}
