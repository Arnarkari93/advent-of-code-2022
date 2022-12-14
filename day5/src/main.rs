fn print_stacks(stacks: &Vec<Vec<u8>>) {
    for (index, stack) in stacks.iter().enumerate() {
        println!(
            "stack {}: {:?}",
            index + 1,
            stack.iter().map(|c| *c as char).collect::<Vec<char>>()
        );
    }
}

fn get_index_as_number(string: &Vec<&str>, i: usize) -> Option<usize> {
    return string.get(i).map(|n| n.parse::<usize>().ok()).flatten();
}

fn print_result(identifier: &str, stacks: &Vec<Vec<u8>>) {
    print!("Result {}:", identifier);
    for stack in stacks {
        print!("{}", *stack.last().unwrap_or(&b' ') as char);
    }
    println!();
}

fn main() {
    let (stacks_section, instructions) = include_str!("../input.txt").split_once("\n\n").unwrap();

    let mut inital_positions = stacks_section.lines().rev();
    let mut stacks_a: Vec<Vec<u8>> = inital_positions
        .next()
        .unwrap()
        .split_whitespace()
        .map(|_| Vec::new())
        .collect::<Vec<Vec<u8>>>();

    let mut stacks_b: Vec<Vec<u8>> = stacks_a.clone();

    for line in inital_positions {
        let crates = line.as_bytes().chunks(4).map(|n| n[1]);
        for (index, c) in crates.enumerate() {
            if c == b' ' {
                continue;
            }

            stacks_a[index].push(c);
            stacks_b[index].push(c);
        }
    }

    for instruction in instructions.lines() {
        // print_stacks(&stacks_b);
        let instruction = instruction.split_whitespace().collect::<Vec<&str>>();
        let how_many = get_index_as_number(&instruction, 1).unwrap_or(0);
        let from_where = get_index_as_number(&instruction, 3).unwrap_or(0);
        let to_where = get_index_as_number(&instruction, 5).unwrap_or(0);

        // a
        // println!("move {} from {} to {}", how_many, from_where, to_where);
        for _ in 0..how_many {
            let move_crate = stacks_a[from_where - 1].pop();
            match move_crate {
                Some(c) => stacks_a[to_where - 1].push(c),
                None => break,
            }
        }

        // b
        let stacks_b_len = stacks_b[from_where -1].len();
        let how_many_max = std::cmp::min(stacks_b_len, how_many);
        let move_crates = stacks_b[from_where - 1].split_off(stacks_b_len - how_many_max);
        move_crates.iter().for_each(|c| stacks_b[to_where - 1].push(*c));
    }

    print_result("a", &stacks_a);
    print_result("b", &stacks_b);
}
