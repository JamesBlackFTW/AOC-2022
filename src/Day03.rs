use core::panic;
use std::collections::HashSet;

fn item_priority(item: char) -> i32 {
    if item.is_lowercase() {
        return item as i32 - 'a' as i32 + 1;
    }    
    return item as i32 - 'A' as i32 + 27;    
}

fn find_duplicates(input: &str) -> i32 {
    let length = input.len();
    let halves = input.split_at(length / 2);

    let set_one = halves.0.chars().collect::<HashSet<char>>();
    let set_two = halves.1.chars().collect::<HashSet<char>>();

    let found_match = *set_one.intersection(&set_two)
        .next()
        .expect("Missing Element");

    return item_priority(found_match);
}

fn find_badge(input: &[String]) -> i32 {
    let found_matches = input.iter()
        .map(|line| line.chars().collect::<HashSet<char>>())
        .reduce(|accum, item| accum.intersection(&item).copied().collect())
        .expect("Something went wrong");

    let found_match = found_matches.iter()
        .next()
        .expect("Missing Element");

    return item_priority(*found_match);
}


pub fn part_one(input: &str) -> Result<i32, i32> {
    let answer = input.lines().map(find_duplicates).sum();
    Ok(answer)
}

pub fn part_two(input: &str) -> Result<i32, i32> {   
    let rucksacks:Vec<String> = input.lines().map(str::to_string).collect();
    
    let answer = rucksacks.chunks(3).map(find_badge).sum();
    Ok(answer)
}

pub fn run_all() {
    let input = include_str!("../data/03.txt");
    println!("Test One: {}", part_one(input).unwrap());
    println!("Test Two: {}", part_two(input).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = include_str!("../samples/03.txt");

    #[test]
    fn sample_part_one() {
        let value = part_one(SAMPLE).unwrap();
        println!("Sample 1: {}", value);
        assert_eq!(value, 157);
    }

    #[test]
    fn sample_part_two() {
        let value = part_two(SAMPLE).unwrap();
        println!("Sample 2: {}", value);
        assert_eq!(value, 70);
    }
}