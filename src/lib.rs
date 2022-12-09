mod Day01;
mod Day02;
mod Day03;
mod Day04;
mod Day05;
mod Day06;

pub fn run(day: &i32 ) {
    match day {
        1 => Day01::run_all(),
        2 => Day02::run_all(),
        3 => Day03::run_all(),
        4 => Day04::run_all(),
        5 => Day05::run_all(),
        6 => Day06::run_all(),
        _ => println!("Could not find solution for Day {}", day)
    }
}