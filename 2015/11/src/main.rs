use std::cmp::Ordering;

fn main() {
    advent_of_code::run(first, second);
}

fn first(input: String) -> String {
    let mut password: [u8; 8] = input.into_bytes().try_into().unwrap();
    next_password(&mut password);
    String::from_utf8(password.to_vec()).unwrap()
}

fn second(input: String) -> String {
    let mut password: [u8; 8] = input.into_bytes().try_into().unwrap();
    next_password(&mut password);
    next_password(&mut password);
    String::from_utf8(password.to_vec()).unwrap()
}

fn next_password(password: &mut [u8; 8]) {
    increase(password, 7);

    if let Some(i) =
        (0..8).find(|&i| password[i] == b'i' || password[i] == b'o' || password[i] == b'l')
    {
        increase(password, i);
    }

    let mut successful = false;

    while !successful {
        let increasing_index = (0..6)
            .find(|&i| password[i] + 1 == password[i + 1] && password[i] + 2 == password[i + 2]);
        let first_pair_index = (0..7).find(|&i| password[i] == password[i + 1]);
        let second_pair_index =
            first_pair_index.and_then(|i| ((i + 2)..7).find(|&i| password[i] == password[i + 1]));

        successful = match (increasing_index, first_pair_index, second_pair_index) {
            (None, None, None)
            | (None, Some(3..), None)
            | (None, Some(3..), Some(5..))
            | (Some(3..), None, None)
            | (Some(3..), Some(3..), None) => try_increasing_pattern(password, [0, 1, 2, 2]),
            (None, Some(..3), None)
            | (None, Some(..3), Some(5..))
            | (Some(5..), Some(..3), None) => try_increasing_pattern(password, [0, 1, 2]),
            (None, Some(_), Some(..5)) => try_increasing_pattern(password, [1, 2]),
            (Some(..3), None, None) | (Some(..3), Some(5..), None) => {
                try_non_increasing_pattern::<true>(password)
            }
            (Some(..5), Some(..5), None) => try_non_increasing_pattern::<false>(password),
            (Some(_), Some(_), Some(_)) => true,
            (_, None, Some(_)) => unreachable!(),
        };
    }
}

fn try_increasing_pattern<const N: usize>(password: &mut [u8; 8], offsets: [u8; N]) -> bool {
    if password[7 - N] > b'x' {
        increase(password, 6 - N);
        return false;
    }

    if (b'g'..=b'o').contains(&password[7 - N]) {
        password[7 - N] = b'p';
        password.iter_mut().skip(8 - N).for_each(|x| *x = b'a');
    }

    for (i, &offset) in offsets.iter().enumerate() {
        match password[8 - N + i].cmp(&(password[7 - N] + offset)) {
            Ordering::Less => break,
            Ordering::Equal => {}
            Ordering::Greater => {
                increase(password, 7 - N + i);
                return false;
            }
        }
    }

    for (i, &offset) in offsets.iter().enumerate() {
        password[8 - N + i] = password[7 - N] + offset;
    }

    true
}

fn try_non_increasing_pattern<const TWO: bool>(password: &mut [u8; 8]) -> bool {
    if TWO && password[5] > password[4] {
        increase(password, 4);
        return false;
    } else if password[7] > password[6] {
        increase(password, 6);
        return false;
    }

    if TWO && password[5] < password[4] {
        password[5] = password[4];
        password[6] = b'a';
        password[7] = b'a';
    } else {
        password[7] = password[6];
    }

    true
}

fn increase(password: &mut [u8; 8], index: usize) {
    password.iter_mut().skip(index + 1).for_each(|x| *x = b'a');

    for i in (0..=index).rev() {
        match password[i] {
            b'z' => {
                password[i] = b'a';
                continue;
            }
            b'h' | b'k' | b'n' => password[i] += 2,
            _ => password[i] += 1,
        }
        break;
    }
}
