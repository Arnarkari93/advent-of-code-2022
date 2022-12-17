use std::{iter::Peekable, str::Lines};

fn sum_of_dir(input: &mut Peekable<Lines>, sum: &mut i32) -> i32 {
    let mut dir_size = 0;
    while let Some(line) = input.next() {
        match line {
            "$ cd .." => break,
            "$ ls" => {
                while let Some(line) = input.peek() {
                    if line.starts_with("$") {
                        break;
                    }
                    if line.starts_with("dir") {
                        input.next();
                        continue;
                    }
                    let file_size = input
                        .next()
                        .map(|i| i.split_once(" "))
                        .flatten()
                        .map(|(s, _f)| s.parse::<i32>().ok())
                        .flatten()
                        .unwrap_or(0);
                    dir_size += file_size;
                }
            }
            _ => {
                dir_size += sum_of_dir(input, sum);
            }
        }
    }
    if dir_size <= 100000 {
        *sum += dir_size;
    }
    return dir_size;
}

fn main() {
    let mut input = include_str!("../input.txt").lines().peekable();
    let mut sum = 0;
    sum_of_dir(&mut input, &mut sum);
    println!("Result: {}", sum);
}
