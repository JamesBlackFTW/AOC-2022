use AOC_2022::run;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();    
    let day = &args[1].parse::<i32>().unwrap();

    run(day);
}