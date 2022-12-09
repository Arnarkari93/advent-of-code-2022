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
        .map(|r| {
            let (r1, r2) = r.split_at(r.len() / 2);
            println!("r1: {}, r2: {}", r1, r2);
            for item in r1.as_bytes() {
                if r2.as_bytes().contains(item) {
                    return char_to_prioritie(item);
                }
            }
            return 0;
        })
        .sum();

    println!("Result sum: {}", result);
}

    

        

        
