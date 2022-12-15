use std::collections::VecDeque;
use std::iter::FromIterator;

fn all_uniq(s: &[char]) -> bool {
    let mut chars = s.to_vec();
    chars.sort();
    chars.dedup();
    chars.len() == s.len()
}

fn recursion_ftw(mut input: VecDeque<char>, index: usize, message_size: usize) -> Option<usize> {
    if input.len() < message_size + 1 {
        return None;
    }
    let next_index = index + 1;

    let front = input.pop_front().unwrap();
    let mut rest = input
        .range(0..(message_size - 1))
        .copied()
        .collect::<VecDeque<char>>();
    let mut message = VecDeque::from([front]);
    message.append(&mut rest);

    if all_uniq(&message.make_contiguous()) {
        return Some(next_index);
    }

    return recursion_ftw(input, next_index, message_size);
}

fn main() {
    let input_lines = include_str!("../input.txt").lines();

    for line in input_lines {
        let input = line.chars();

        let result_a = recursion_ftw(VecDeque::from_iter(input.clone().into_iter()), 3, 4);
        let result_b = recursion_ftw(VecDeque::from_iter(input.clone().into_iter()), 13, 14);

        println!("Result A:{}", result_a.unwrap_or(0));
        println!("Result B:{}", result_b.unwrap_or(0));
    }
}
