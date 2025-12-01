mod dial;
use dial::Dial;
use dial::Direction;

use crate::util;


pub fn solve() {
    println!("Solving day 1...");

    let testfile_name: &str = "./src/day01/input.csv";
    const STARTING_POINT: i32 = 50;
    const MAX_NUM: i32 = 99;

    let mut input: Vec<Vec<String>> = Vec::new();
    let mut parsed: Vec<(Direction, i32)> = Vec::new();
    util::csv::parse_csv_file(testfile_name.to_string(), &mut input);

    for v in input {
        for e in v {
            let calc = parse(e);

            parsed.push(calc);
        }
    }

    let mut dial: dial::Dial =  Dial::new(MAX_NUM, STARTING_POINT);
    
    _silver_star(&mut dial, &parsed);
    dial.reset();
    _gold_star(&mut dial, &parsed);
}

pub fn _silver_star(dial: &mut Dial, instructions: &Vec<(Direction, i32)>) {
    println!("\tSolving silver star...");
    for inst in instructions {
        dial.print();
        dial.turn(&inst.0, inst.1);
    }
    dial.print();

    println!("\tZeros hit: {:?}", dial.zeros_hit);
}
pub fn _gold_star(dial: &mut Dial, instructions: &Vec<(Direction, i32)>) {
    println!("\tSolving gold star...");
    for inst in instructions {
        dial.print();
        dial.turn_with_overflowcalc(&inst.0, inst.1);
    }
    dial.print();

    println!("\tZeros hit: {:?}", dial.zeros_hit);
}

pub fn parse(input: String) -> (Direction, i32) {
    let mut sstring = input.split_at(1);
    let mut dir = Direction::LEFT;

    if sstring.0 == "R" {
        dir = Direction::RIGHT;
    }

    let clicks: i32 = sstring.1.parse().expect("Failed to parse number!");


    return (dir, clicks);
}
