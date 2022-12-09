enum Choice {
    Rock,
    Paper,
    Scissors,
}

enum Outcome {
    Win,
    Loose,
    Draw,
}

fn choice_to_points(choice: &Choice) -> u32 {
    let result = match choice {
        Choice::Rock => 1,
        Choice::Paper => 2,
        Choice::Scissors => 3,
    };

    return result;
}

fn play_rps(their_choice: &Choice, our_choice: &Choice) -> Outcome {
    use crate::Choice::*;
    use crate::Outcome::*;
    let result = match (their_choice, our_choice) {
        (Rock, Paper) => Win,
        (Rock, Scissors) => Loose,
        (Rock, Rock) => Draw,
        (Paper, Paper) => Draw,
        (Paper, Scissors) => Win,
        (Paper, Rock) => Loose,
        (Scissors, Paper) => Loose,
        (Scissors, Scissors) => Draw,
        (Scissors, Rock) => Win,
    };

    return result;
}

fn outcome_to_points(action: &Outcome) -> u32 {
    use crate::Outcome::*;
    let result = match action {
        Win => 6,
        Loose => 0,
        Draw => 3,
    };

    return result;
}

fn decode_choice(choice: &str) -> Choice {
    let result = match choice {
        "A" | "X" => Choice::Rock,
        "B" | "Y" => Choice::Paper,
        "C" | "Z" => Choice::Scissors,
        invalid => panic!("Invalid choice: {}", invalid),
    };

    return result;
}

fn decode_action(choice: &str) -> Outcome {
    use crate::Outcome::*;
    let result = match choice {
        "X" => Loose,
        "Y" => Draw,
        "Z" => Win,
        invalid => panic!("Invalid choice: {}", invalid),
    };

    return result;
}

fn a(rounds: &Vec<(&str, &str)>) {
    let mut score = 0;
    for (them, we) in rounds {
        let their_choice = decode_choice(them);
        let our_choice = decode_choice(we);

        score = score + choice_to_points(&our_choice);
        let outcome = play_rps(&their_choice, &our_choice);
        score = score + outcome_to_points(&outcome);
    }
    println!("Score: {}", score);
}

fn b(rounds: &Vec<(&str, &str)>) {
    use crate::Choice::*;
    use crate::Outcome::*;

    let mut score = 0;
    for (them, we) in rounds {
        let their_choice = decode_choice(them);
        let desired_outcome = decode_action(we);

        match (their_choice, &desired_outcome) {
            (Rock, Loose) => score = score + choice_to_points(&Scissors),
            (Rock, Draw) => score = score + choice_to_points(&Rock),
            (Rock, Win) => score = score + choice_to_points(&Paper),
            (Paper, Loose) => score = score + choice_to_points(&Rock),
            (Paper, Draw) => score = score + choice_to_points(&Paper),
            (Paper, Win) => score = score + choice_to_points(&Scissors),
            (Scissors, Loose) => score = score + choice_to_points(&Paper),
            (Scissors, Draw) => score = score + choice_to_points(&Scissors),
            (Scissors, Win) => score = score + choice_to_points(&Rock),
        }
        score = score + outcome_to_points(&desired_outcome);
    }
    println!("Score: {}", score);
}

fn main() {
    let input = include_str!("../input.txt").lines();
    let rounds = input
        .map(|round| round.split_once(" ").unwrap())
        .collect::<Vec<(&str, &str)>>();

    a(&rounds);
    b(&rounds);
}
