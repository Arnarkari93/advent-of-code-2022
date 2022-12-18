use std::{iter::Peekable, str::Lines};

struct Dir(Vec<Dir>, i32, String);

impl Dir {
    fn size(&self) -> i32 {
        let children_size = self.0.iter().map(|d| d.size()).sum::<i32>();
        let dir_size = children_size + self.1;
        dir_size
    }
}

fn size_of_files(input: &mut Peekable<Lines>, dir_size: &mut i32) {
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
        *dir_size += file_size;
    }
}

fn input_to_dir(input: &mut Peekable<Lines>) -> Dir {
    let mut dirs = vec![];
    let mut dir_size = 0;
    let mut dir_name = "/";
    while let Some(line) = input.next() {
        match line {
            "$ cd .." => break,
            "$ ls" => size_of_files(input, &mut dir_size),
            _ => {
                let dir = input_to_dir(input);
                dir_name = line;
                dirs.push(dir);
            }
        }
    }
    return Dir(dirs, dir_size, dir_name.to_string());
}

fn a_sum(dir: &Dir, sum: &mut i32) -> i32 {
    let children_size = dir.0.iter().map(|d| a_sum(d, sum)).sum::<i32>();
    let dir_size = children_size + dir.1;
    if dir_size <= 100000 {
        *sum += dir_size;
    }

    return dir_size;
}

fn b_size(dir: &Dir, size: &mut i32, space_needed: i32) -> i32 {
    let dir_size = dir
        .0
        .iter()
        .map(|d| b_size(d, size, space_needed))
        .sum::<i32>()
        + dir.1;

    if dir_size >= space_needed && *size >= dir_size {
        *size = dir_size;
    }
    return dir_size;
}

static DISK_SIZE: i32 = 70000000;
static UPDATE_SIZE: i32 = 30000000;
fn get_space_needed(used_space: i32) -> i32 {
    let free_space = DISK_SIZE - used_space;
    let space_needed = UPDATE_SIZE - free_space;
    space_needed
}

fn main() {
    let input = include_str!("../input.txt").lines().peekable();
    let mut size = 0;
    let root_dir = input_to_dir(&mut input.clone());
    a_sum(&root_dir, &mut size);
    println!("Result a: {}", size);

    let root_dir = input_to_dir(&mut input.clone());
    let used_space = root_dir.size();
    let mut size = i32::MAX;
    b_size(&root_dir, &mut size, get_space_needed(used_space));
    println!("Result b: {}", size);
}
