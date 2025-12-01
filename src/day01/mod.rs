use std::error::Error;
use csv::Reader;

use crate::util;

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    LEFT,
    RIGHT
}

pub fn solve() {
    _silver_star();
    _gold_star();
}

pub fn _silver_star() {
    println!("Solving day 1...");

    let testfile_name: &str = "./src/day01/test.csv";
    const starting_point: i32 = 50;
    const max_num: i32 = 100;

    let mut runner: i32 = starting_point;
    let mut counter: i32 = 0;

    let mut input: Vec<Vec<String>> = Vec::new();
    let mut parsed: Vec<(Direction, i32)> = Vec::new();
    util::csv::parse_csv_file(testfile_name.to_string(), &mut input);

    for v in input {
        for e in v {
            let calc = parse(e);

            parsed.push(calc);
        }
    }

    for p in parsed {
        if p.0 == Direction::LEFT {
            runner = ((runner - p.1) + max_num) % max_num
        }else{
            runner = ((runner + p.1) + max_num) % max_num
        }

        if runner == 0 {
            counter = counter + 1;
        }
    }

    println!("Silver: Found zero {:?} times...", counter);
}

pub fn _gold_star() {
    println!("Solving day 1...");

    let testfile_name: &str = "./src/day01/input.csv";
    const starting_point: i32 = 50;
    const max_num: i32 = 100;

    let mut runner: i32 = starting_point;
    let mut counter: i32 = 0;

    let mut input: Vec<Vec<String>> = Vec::new();
    let mut parsed: Vec<(Direction, i32)> = Vec::new();
    util::csv::parse_csv_file(testfile_name.to_string(), &mut input);

    for v in input {
        for e in v {
            let calc = parse(e);

            parsed.push(calc);
        }
    }

    for p in parsed {
        let mut prerunner: i32 = runner;
        if p.0 == Direction::LEFT {
            runner = wrap_calc(prerunner - p.1, max_num);
            let wraps = (prerunner - p.1).div_euclid(max_num).abs();

            println!("{:?} - {:?} -> {:?} [{:?}]", prerunner, p.1, prerunner - p.1 < 0, runner);
            if prerunner - p.1 < 0 && runner != 0 && prerunner != 0 {
                println!("w: {:?}", wraps);
                if wraps == 0 {
                    counter = counter + 1;
                }else {
                    counter = counter + wraps;
                }
            }
        }else{
            runner =  wrap_calc(prerunner + p.1, max_num);
            let wraps = (prerunner + p.1).div_euclid(max_num).abs();
            
            println!("{:?} + {:?} -> {:?} [{:?}]", prerunner, p.1, prerunner + p.1 >= max_num, runner);
            if prerunner + p.1 >= max_num && runner != 0 && prerunner != 0 {
                println!("w: {:?}", wraps);
                if wraps == 0 {
                    counter = counter + 1;
                }else {
                    counter = counter + wraps;
                }
            }
        }

        if runner == 0 {
            counter = counter + 1;
        }
    }

    println!("Gold Star: Found zero {:?} times...", counter);
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


pub fn wrap_calc(num: i32, m: i32) -> i32 {
    let mut otimes = 0;
    if num < 0 {
        otimes = (-num / m) + 1;
    }

    return (num + m * otimes) % m;
}