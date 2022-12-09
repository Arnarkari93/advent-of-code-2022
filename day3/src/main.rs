use std::cmp::PartialEq;

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
        println!("found {}: {}", *c as char, (c - b'a') as i32);
        return (c - b'a') as i32 + 1;
    }
    println!("found {}: {}", *c as char, (c - b'A') as i32 + 27);
    return (c - b'A') as i32 + 27;
}

fn main() {
    let ruckstack = include_str!("../input.txt").lines();
    let result: i32 = ruckstack
        .map(|r| r.split_at(r.len() / 2))
        .map(|(r1, r2)| {
            println!("r1: {}, r2: {}", r1, r2);
            return match contains_same_element(r1.as_bytes(), r2.as_bytes()) {
                Ok(c) => char_to_prioritie(c),
                Err(_) => 0,
            };
        })
        .sum();

    println!("Result sum: {}", result);
}
