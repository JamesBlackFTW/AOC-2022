fn get_calorie_list(input: &str) -> Vec<i32> {
    let mut elves = input.split("\r\n\r\n").map (
        |elf| elf.lines().map ( 
            |cal| cal.parse::<i32>().unwrap()
        )        
        .sum::<i32>()
    ).collect::<Vec<i32>>();

    elves.sort_by(|a, b| b.cmp(a));
    
    return elves;
}

pub fn part_one(input: &str) -> Result<i32, i32> {
    let elves = get_calorie_list(input);
    Ok(*elves.iter().max().unwrap())
}

pub fn part_two(input: &str) -> Result<i32, i32> {
    let elves = get_calorie_list(input);
    Ok(elves.iter().take(3).sum::<i32>())
}

pub fn run_all() {
    let input = include_str!("../data/01.txt");
    println!("Test One: {}", part_one(input).unwrap());
    println!("Test Two: {}", part_two(input).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = include_str!("../samples/01.txt");

    #[test]
    fn sample_part_one() {
        let value = part_one(SAMPLE).unwrap();
        println!("Sample 1: {}", value);
        assert_eq!(value, 24000);
    }

    #[test]
    fn sample_part_two() {
        let value = part_two(SAMPLE).unwrap();
        println!("Sample 2: {}", value);
        assert_eq!(value, 45000);
    }
}