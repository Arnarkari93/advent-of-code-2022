use std::cmp::PartialEq;
use std::str::Lines;

fn contains_same_element<'element, T>(
    list1: &'element [T],
    list2: &[T],
) -> Result<&'element T, &'static str>
where
    T: PartialEq,
{
    for element in list1 {
        if list2.contains(element) {
            let result = element.to_owned();
            return Ok(result);
        }
    }
    return Err("No common element found");
}

fn char_to_prioritie(c: &u8) -> i32 {
    if *c >= b'a' {
        let result = (c - b'a') as i32 + 1;
        println!("found {}: {}", *c as char, result);
        return result;
    }

    let result = (c - b'A') as i32 + 27;
    println!("found {}: {}", *c as char, result);
    result
}

fn part_a(ruckstacks: &Lines) {
    let result: i32 = ruckstacks
        .clone()
        .map(|r| r.split_at(r.len() / 2))
        .map(|(r1, r2)| {
            println!("r1: {}, r2: {}", r1, r2);
            return match contains_same_element(r1.as_bytes(), r2.as_bytes()) {
                Ok(c) => char_to_prioritie(c),
                Err(_) => 0,
            };
        })
        .sum();

    println!("Part A result sum: {}", result);
}

fn part_b(ruckstacks: &Lines) {
    let ruckstacks = ruckstacks.clone();
    let result: i32 = ruckstacks
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|group| {
            println!("group: {:?}", group);
            let index = group[0].find(|c| group[1].contains(c) && group[2].contains(c));
            return match index {
                Some(index) => char_to_prioritie(&group[0].as_bytes()[index]),
                None => 0,
            };
        })
        .sum();
    println!("Part B result sum: {}", result);
}

fn main() {
    let ruckstacks = include_str!("../input.txt").lines();

    part_a(&ruckstacks);
    part_b(&ruckstacks);
}
