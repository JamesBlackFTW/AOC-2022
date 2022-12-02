use core::panic;

enum Sign {
    Unknown = 0,
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

enum Outcome {
    Unknown = 0,
    Win = 1,
    Draw = 2,
    Lose = 3,
}

fn convert_score(input: char) -> Sign {
    match input {
        'A' | 'X' => return Sign::Rock,
        'B' | 'Y' => return Sign::Paper,
        'C' | 'Z' => return Sign::Scissors,
        _ => panic!()
    }
}

fn convert_outcome(input: char) -> Outcome {
    match input {
        'X' => return Outcome::Lose,
        'Y' => return Outcome::Draw,
        'Z' => return Outcome::Win,
        _ => panic!()
    }
}

fn calculate_loss(myself: Sign) -> i32 {
    return myself as i32 + 0
}

fn calculate_draw(myself: Sign) -> i32 {
    return myself as i32 + 3
}

fn calculate_win(myself: Sign) -> i32 {
    return myself as i32 + 6
}

fn calculate_score(input: &str) -> Result<i32, i32> {
    let moves: Vec<char> = input.chars().collect();
    let opponent = convert_score(moves[0]);
    let myself = convert_score(moves[2]);

    match (opponent, myself) {
        (Sign::Rock, Sign::Rock) => return Ok(calculate_draw(Sign::Rock)),
        (Sign::Rock, Sign::Paper) => return Ok(calculate_win(Sign::Paper)),
        (Sign::Rock, Sign::Scissors) => return Ok(calculate_loss(Sign::Scissors)),
        (Sign::Paper, Sign::Rock) => return Ok(calculate_loss(Sign::Rock)),
        (Sign::Paper, Sign::Paper) => return Ok(calculate_draw(Sign::Paper)),
        (Sign::Paper, Sign::Scissors) => return Ok(calculate_win(Sign::Scissors)),
        (Sign::Scissors, Sign::Rock) => return Ok(calculate_win(Sign::Rock)),
        (Sign::Scissors, Sign::Paper) => return Ok(calculate_loss(Sign::Paper)),
        (Sign::Scissors, Sign::Scissors) => return Ok(calculate_draw(Sign::Scissors)),
        _ => return Ok(0)
    }
}

fn calculate_strategy_score(input: &str) -> Result<i32, i32> {
    let moves: Vec<char> = input.chars().collect();
    let opponent = convert_score(moves[0]);
    let myself = convert_outcome(moves[2]);

    match (opponent, myself) {
        (Sign::Rock, Outcome::Win) => return Ok(calculate_win(Sign::Paper)),
        (Sign::Rock, Outcome::Draw) => return Ok(calculate_draw(Sign::Rock)),
        (Sign::Rock, Outcome::Lose) => return Ok(calculate_loss(Sign::Scissors)),
        (Sign::Paper, Outcome::Win) => return Ok(calculate_win(Sign::Scissors)),
        (Sign::Paper, Outcome::Draw) => return Ok(calculate_draw(Sign::Paper)),
        (Sign::Paper, Outcome::Lose) => return Ok(calculate_loss(Sign::Rock)),
        (Sign::Scissors, Outcome::Win) => return Ok(calculate_win(Sign::Rock)),
        (Sign::Scissors, Outcome::Draw) => return Ok(calculate_draw(Sign::Scissors)),
        (Sign::Scissors, Outcome::Lose) => return Ok(calculate_loss(Sign::Paper)),
        _ => return Ok(0)
    }
}


pub fn part_one(input: &str) -> Result<i32, i32> {

    let mut result = input.lines().map(|line| calculate_score(line).unwrap()).sum();
    println!("Overall {0}", result);
    Ok(result)
}

pub fn part_two(input: &str) -> Result<i32, i32> {
    let mut result = input.lines().map(|line| calculate_strategy_score(line).unwrap()).sum();
    println!("Overall {0}", result);
    Ok(result)
}

pub fn run_all() {
    let input = include_str!("../data/02.txt");
    println!("Test One: {}", part_one(input).unwrap());
    println!("Test Two: {}", part_two(input).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = include_str!("../samples/02.txt");

    #[test]
    fn sample_part1() {
        assert_eq!(part_one(SAMPLE).unwrap(), 15);
    }

    #[test]
    fn sample_part2() {
        assert_eq!(part_two(SAMPLE).unwrap(), 12);
    }
}