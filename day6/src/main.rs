use std::collections::VecDeque;
use std::iter::FromIterator;

fn all_uniq(s: &[char]) -> bool {
    let mut chars = s.to_vec();
    chars.sort();
    chars.dedup();
    chars.len() == s.len()
}

fn recursion_ftw(mut input: VecDeque<char>, index: usize) -> Option<usize> {
    if input.len() < 4 {
        return None;
    }
    let next_index = index + 1;

    let (a, b, c, d) = (input.pop_front().unwrap(), input[0], input[1], input[2]);
    println!("Step:  {}{}{}{}", a, b, c, d);

    if all_uniq(&[a, b, c, d]) {
        return Some(next_index);
    }

    return recursion_ftw(input, next_index);
}

fn main() {
    let input_lines = include_str!("../input.txt").lines();

    for line in input_lines {
        let input = line.chars();
        println!("input: {}", line);
        let output = recursion_ftw(VecDeque::from_iter(input.into_iter()), 3);
        println!("Result {}", output.unwrap_or(0));
    }
}
