fn main() {
    advent_of_code::run(first, second);
}

fn first(input: String) -> u32 {
    let ingredients = parse_ingredients(&input);
    solve::<false>(&ingredients, 0, 100, &mut [0; 5])
}

fn second(input: String) -> u32 {
    let ingredients = parse_ingredients(&input);
    solve::<true>(&ingredients, 0, 100, &mut [0; 5])
}

fn parse_ingredients(input: &str) -> Vec<[i32; 5]> {
    input
        .lines()
        .map(|x| {
            x.split(',')
                .map(|x| x.rsplit(' ').next().unwrap().parse().unwrap())
                .collect::<Vec<i32>>()
                .try_into()
                .unwrap()
        })
        .collect()
}

fn solve<const CALORIES: bool>(
    ingredients: &[[i32; 5]],
    current_ingredient: usize,
    left_amount: u8,
    properties: &mut [i32; 5],
) -> u32 {
    if current_ingredient == ingredients.len() - 1 {
        let calories = properties[4] + (left_amount as i32) * ingredients[current_ingredient][4];
        if CALORIES && calories != 500 {
            return 0;
        }

        return properties[..4]
            .iter()
            .zip(ingredients[current_ingredient])
            .map(|x| (x.0 + (left_amount as i32) * x.1).max(0) as u32)
            .product();
    }

    let mut max = solve::<CALORIES>(ingredients, current_ingredient + 1, left_amount, properties);

    for amount in 1..=left_amount {
        (0..5).for_each(|j| properties[j] += ingredients[current_ingredient][j]);

        max = max.max(solve::<CALORIES>(
            ingredients,
            current_ingredient + 1,
            left_amount - amount,
            properties,
        ));
    }

    (0..5).for_each(|j| properties[j] -= (left_amount as i32) * ingredients[current_ingredient][j]);

    max
}
