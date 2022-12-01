mod Day01;

pub fn run(day: &i32 ) {
    match day {
        1 => Day01::run_all(),
        _ => println!("Could not find solution for Day {}", day)
    }
}