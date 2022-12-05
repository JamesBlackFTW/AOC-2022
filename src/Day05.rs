use itertools::Itertools;
use regex::Regex;

fn convert_stack_input(split_input: &str) -> Vec<Vec<char>> {
    let mut stacks = Vec::new();   
    for _ in 0..9 {
        let stack = Vec::new();   
        stacks.push(stack);
    }

    split_input.lines().filter(|line|
        !line.starts_with(" 1")
    )
    .for_each(|line| {        
        for stack_index in 0..9 {
            let crate_value = line.chars().nth(1 + (4 * stack_index));          
            if crate_value.is_some() {
                let crate_identity = crate_value.unwrap();
                if crate_identity != ' ' {
                    stacks[stack_index].push(crate_identity)
                }
            }       
        }
    });

    return stacks;
}

fn perform_movement(instructions: &str, stacks: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let lines = instructions.lines();
    let mut new_stack = stacks.clone();
    let a = lines.collect_vec();

    a.iter().for_each(|line| {
        println!("{}", line);

        let re = Regex::new(r"[a-z\s]{5}(\d*)[a-z\s]{6}(\d*)[a-z\s]{4}(\d*)").unwrap();
        let caps = re.captures(line).unwrap();

        let number_to_move = caps.get(1).map_or(0, |x| x.as_str().parse::<i32>().unwrap());
        let from_stack = caps.get(2).map_or(0, |x| x.as_str().parse::<i32>().unwrap()) - 1;
        let to_stack = caps.get(3).map_or(0, |x| x.as_str().parse::<i32>().unwrap()) - 1;

        for _ in 0..number_to_move {
            let crate_to_move = new_stack[from_stack as usize].remove(0);
            new_stack[to_stack as usize].insert(0, crate_to_move)
        }
    });

    return new_stack;
}

fn perform_advanced_movement(instructions: &str, stacks: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let lines = instructions.lines();
    let mut new_stack = stacks.clone();
    let a = lines.collect_vec();

    a.iter().for_each(|line| {
        let re = Regex::new(r"[a-z\s]{5}(\d*)[a-z\s]{6}(\d*)[a-z\s]{4}(\d*)").unwrap();
        let caps = re.captures(line).unwrap();

        let number_to_move = caps.get(1).map_or(0, |x| x.as_str().parse::<i32>().unwrap());
        let from_stack = caps.get(2).map_or(0, |x| x.as_str().parse::<i32>().unwrap()) - 1;
        let to_stack = caps.get(3).map_or(0, |x| x.as_str().parse::<i32>().unwrap()) - 1;

        let mut temp_array:Vec<char> = Vec::new();
        for _ in 0..number_to_move {
            let crate_to_move = new_stack[from_stack as usize].remove(0);
            temp_array.push(crate_to_move);
        }

        for _ in 0..number_to_move {
            let crate_to_move = temp_array.pop().unwrap();
            new_stack[to_stack as usize].insert(0, crate_to_move)
        }
    });

    return new_stack;
}

fn concatenate_stack(stacks: Vec<Vec<char>>) -> String {
    return stacks.into_iter().filter(
        |stack| stack.len() > 0
    ).map(|stack| 
        stack[0]
    ).filter(|crate_identifier| 
        *crate_identifier!=' '
    ).collect();
}

pub fn part_one(input: &str) -> Result<String, String> {  
    let split_input = input.split_once("\r\n\r\n").unwrap();

    let stacks = convert_stack_input(split_input.0);    
    let new_stack = perform_movement(split_input.1, stacks); 
    Ok(concatenate_stack(new_stack))
}

pub fn part_two(input: &str) -> Result<String, String> {  
    let split_input = input.split_once("\r\n\r\n").unwrap();

    let stacks = convert_stack_input(split_input.0);    
    let new_stack = perform_advanced_movement(split_input.1, stacks); 
    Ok(concatenate_stack(new_stack))
}

pub fn run_all() {
    let input = include_str!("../data/05.txt");
    println!("Test One: {}", part_one(input).unwrap());
    println!("Test Two: {}", part_two(input).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = include_str!("../samples/05.txt");

    #[test]
    fn sample_part_one() {
        let value = part_one(SAMPLE).unwrap();
        println!("Sample 1: {}", value);
        assert_eq!(value, "CMZ".to_string());
    }

    #[test]
    fn sample_part_two() {
        let value = part_two(SAMPLE).unwrap();
        println!("Sample 2: {}", value);
        assert_eq!(value, "MCD".to_string());
    }
}