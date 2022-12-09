use std::collections::HashSet;

use itertools::Itertools;
use regex::Regex;

fn find_unique(input: &str, size: usize) -> i32 {
    let first_unique = input
    .as_bytes()
    .windows(size)
    .enumerate()
    .filter(|(index, subset)| {
        let hs:HashSet<u8> = HashSet::from_iter(subset.iter().cloned());
        return hs.len() == size;
    })
    .map(|(index, _)| index as i32)
    .next()
    .unwrap();
    
    return first_unique + size as i32;
}

pub fn part_one(input: &str) -> i32 {    
    return find_unique(input, 4);
}

pub fn part_two(input: &str) -> i32 {  
    return find_unique(input, 14);
}

pub fn run_all() {
    let input = include_str!("../data/06.txt");
    println!("Test One: {}", part_one(input));
    println!("Test Two: {}", part_two(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = include_str!("../samples/06.txt");

    #[test]
    fn sample_part_one() {
        let value = part_one(SAMPLE);
        println!("Sample 1: {}", value);
        assert_eq!(value, 5);
    }

    #[test]
    fn sample_part_two() {
        let value = part_two(SAMPLE);
        println!("Sample 2: {}", value);
        assert_eq!(value, 23);
    }
}