use std::fs;

#[derive(PartialEq, Clone, Copy, Debug)] 
enum Move {
    Rock,
    Paper,
    Scissors
}

fn parse_char(action: char) -> Move {
    match action {
        'A' => Move::Rock,
        'B' => Move::Paper,
        'C' => Move::Scissors,
        'X' => Move::Rock,
        'Y' => Move::Paper,
        'Z' => Move::Scissors,
        _ => todo!()
    }
}

fn beats(action: Move) -> Move {
    // Looks reversed here, but look at how nicely it now reads in the score function
    match action {
        Move::Rock => Move::Paper,
        Move::Paper => Move::Scissors,
        Move::Scissors => Move::Rock
    }
}

fn choice_score(yours: Move) -> usize {
    match yours {
        Move::Rock => 1,
        Move::Paper => 2,
        Move::Scissors => 3
    }
}

fn win_score(yours: Move, theirs: Move) -> usize {
    match (yours, theirs) {
        (yours, theirs) if yours  == beats(theirs) => 6,
        (yours, theirs) if yours  == theirs        => 3,
        (yours, theirs) if theirs == beats(yours)  => 0,
        (_, _) => todo!()
    }
}

fn main() {
    let file = fs::read_to_string("inputs/day2.txt").unwrap();
    
    let lines = file.split("\n");

    let total_score: usize = lines.fold(0, |acc, line| {
        let yours = parse_char(line.as_bytes()[2] as char);
        let theirs = parse_char(line.as_bytes()[0] as char);
        // println!("==> {:?}", acc);
        println!("{:?} vs {:?} -> {:?} + {:?}", yours, theirs, win_score(yours, theirs), choice_score(yours));
        acc + win_score(yours, theirs) + choice_score(yours)
    });

    println!("Total score: {}", total_score);
}
