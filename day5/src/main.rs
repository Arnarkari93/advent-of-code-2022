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

fn main() {
    let (stacks_section, instructions) = include_str!("../input.txt").split_once("\n\n").unwrap();

    let mut inital_positions = stacks_section.lines().rev();
    let mut stacks: Vec<Vec<u8>> = inital_positions
        .next()
        .unwrap()
        .split_whitespace()
        .map(|_| Vec::new())
        .collect::<Vec<Vec<u8>>>();

    for line in inital_positions {
        let crates = line.as_bytes().chunks(4).map(|n| n[1]);
        for (index, c) in crates.enumerate() {
            if c == b' ' {
                continue;
            }

            stacks[index].push(c);
        }
    }

    // print_stacks(&stacks);
    for instruction in instructions.lines() {
        let instruction = instruction.split_whitespace().collect::<Vec<&str>>();
        let how_many = get_index_as_number(&instruction, 1).unwrap_or(0);
        let from_where = get_index_as_number(&instruction, 3).unwrap_or(0);
        let to_where = get_index_as_number(&instruction, 5).unwrap_or(0);

        for _ in 0..how_many {
            let move_crate = stacks[from_where - 1].pop();
            match move_crate {
                Some(c) => stacks[to_where - 1].push(c),
                None => break,
            }
        }
        // println!("move {} from {} to {}", how_many, from_where, to_where);
        // print_stacks(&stacks);
    }

    for stack in stacks {
        print!("{}", *stack.last().unwrap_or(&b' ') as char);
    }
    println!();
}
