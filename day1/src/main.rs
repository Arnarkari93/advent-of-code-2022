fn main() {
    let result = include_str!("../input.txt").split("\n\n").map(|bag| {
        let calories = bag.lines().map(|c| c.parse::<u32>().unwrap()).sum::<u32>();
        return calories;
    });


    let mut sum_of_calories = result.collect::<Vec<u32>>();
    sum_of_calories.sort_by(|a, b| b.cmp(a));

    let result_a = sum_of_calories[0];
    let result_b = sum_of_calories[0..3].to_vec().iter().sum::<u32>();

    println!("Result A: {}", result_a);
    println!("Result B: {}", result_b);
}
