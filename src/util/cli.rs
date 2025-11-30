use std::env;

pub fn parse_args() -> Vec<String> {
    let args: Vec<String> = env::args().collect();

    return args;
}

pub fn get_day(args: &Vec<String>, fallback: i8) -> i8 {
    if args.len() == 2 {
        let day_str: &String = &args[1];
        let optional_day: Option<i8> = match day_str.parse::<i8>() {
            Ok(value) => Some(value),
            Err(_) => None,
        };

        if optional_day.is_some() {
           let day = optional_day.unwrap();

            if day > 0 && day <= 12 {
                return day;
            }
        }
    }

    return fallback;
}

fn print_help_str() {
    println!(
        r#"Advent of Code 2025 Framework!
Usage: cargo run [day]

[day] specifies the day to execute. If no day is given, the constant from main.rs will be used!"#
    );
}