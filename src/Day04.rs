use itertools::Itertools;

pub fn part_one(input: &str) -> Result<i32, i32> {

    let contain_count = input.lines().map(|line|
        line.split(",").map(|elf| 
            elf.split("-").map(|pos|
                pos.parse::<i32>().unwrap()       
            )
        ).flatten().collect_tuple::<(i32, i32, i32, i32)>()
        .unwrap()
    )
    .filter(|(s1, s2, e1, e2)| (s1 <= e1 && s2 >= e2) || (s1 <= e1 && s2 >= e2))
    .count();    
    
    Ok(contain_count as i32)
}

pub fn part_two(input: &str) -> Result<i32, i32> {   
    let overlap_count = input.lines().map(|line|
        line.split(",").map(|elf| 
            elf.split("-").map(|pos|
                pos.parse::<i32>().unwrap()       
            )
        ).flatten().collect_tuple::<(i32, i32, i32, i32)>()
        .unwrap()
    )
    .filter(|(s1, s2, e1, e2)| (s1 <= e1 && e1 <= s2) || (e1 <= s1 && s1 <= e2))
    .count();    

    Ok(overlap_count as i32)
}

pub fn run_all() {
    let input = include_str!("../data/04.txt");
    println!("Test One: {}", part_one(input).unwrap());
    println!("Test Two: {}", part_two(input).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = include_str!("../samples/04.txt");

    #[test]
    fn sample_part_one() {
        let value = part_one(SAMPLE).unwrap();
        println!("Sample 1: {}", value);
        assert_eq!(value, 2);
    }

    #[test]
    fn sample_part_two() {
        let value = part_two(SAMPLE).unwrap();
        println!("Sample 2: {}", value);
        assert_eq!(value, 4);
    }
}