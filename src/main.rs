///
/// Main entrypoint of the AoC framework.
/// Define the day to execute in the constant below.
///


// Day selector
const DAYTOEXECUTE: i8 = 1;

// Add the day modules here...
mod day01;
mod day_xx;
mod util;

fn main() {
    let args: Vec<String> = util::cli::parse_args();
    let day: i8 = util::cli::get_day(&args, DAYTOEXECUTE);

    match day {
        1 => day01::solve(),
        _ => println!("Day not implemented yet"),
    }
}