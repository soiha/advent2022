use std::env;
use std::fs;

#[allow(dead_code)]

// mod day01;

const DEFAULT_INPUT_FILE_DIR: &str = "inputs/";

fn main() {
    let days: Vec<fn(String)->String> = vec![
        crate::day01::day01,
    ];

    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("Usage: {} [day] {{input}}\n", args[0]);
        println!("The day argument must be from 1 to {}.\n", days.len());
        println!("If input is not given, a predefined input file according to the day number will be");
        println!("read from a hardcoded directory (currently {}).", DEFAULT_INPUT_FILE_DIR);
        return;
    } 
    
    let day = args[1].parse().unwrap_or(0);
    
    if day < 1 || day > days.len() {
        println!("Invalid day {}. Allowed values are between 1 and {}.", args[1], days.len());
        return;
    }
    
    let mut file: String = format!("{}input{:0>2}.txt", DEFAULT_INPUT_FILE_DIR, day);
    
    if args.len() >= 3 {
        file = String::from(args[2].as_str());
    }
    
    println!("Running day {} using file {}", day, file);
    
    let input = fs::read_to_string(&file)
        .expect(format!("Could not read input file {}", file).as_str());
    let result = days[day-1](input);
    
    print!("{}", result);
}