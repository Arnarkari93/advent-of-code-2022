use std::{iter::Map, str::Lines};

fn process_input_to_pairs<'r>(
    input: &Lines<'r>,
) -> Map<Lines<'r>, impl FnMut(&'r str) -> ((i32, i32), (i32, i32)) + 'r> {
    return input.to_owned().map(|pair: &str| {
        let pair = pair
            .split(",")
            .map(|sections| {
                let (from, to) = sections.split_once("-").unwrap();
                return (from.parse::<i32>().unwrap(), to.parse::<i32>().unwrap());
            })
            .collect::<Vec<(i32, i32)>>();
        return (pair[0], pair[1]);
    });
}

fn main() {
    let input = include_str!("../input.txt").lines();
    let result_a: i32 = process_input_to_pairs(&input)
        .map(|(a, b): ((i32, i32), (i32, i32))| {
            if a.0 <= b.0 && a.1 >= b.1 {
                println!("{}-{},{}-{}: a contians b", a.0, a.1, b.0, b.1);
                return 1;
            }
            if b.0 <= a.0 && b.1 >= a.1 {
                println!("{}-{},{}-{}: b contians a", a.0, a.1, b.0, b.1);
                return 1;
            }

            println!("{}-{},{}-{}: nada", a.0, a.1, b.0, b.1);
            return 0;
        })
        .sum();
    println!("Result a: {}", result_a);

    let result_b: i32 = process_input_to_pairs(&input)
        .map(|(a, b): ((i32, i32), (i32, i32))| {
            if a.1 >= b.0 && b.1 >= a.0 {
                println!("{}-{},{}-{}: a contians b", a.0, a.1, b.0, b.1);
                return 1;
            }

            println!("{}-{},{}-{}: nada", a.0, a.1, b.0, b.1);
            return 0;
        })
        .sum();
    println!("Result b: {}", result_b);
}
