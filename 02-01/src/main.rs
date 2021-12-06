use clap::{App, Arg};
use std::fs;

fn main() {
    let matches = App::new("aoc-02-01")
        .arg(Arg::with_name("input").short("i").takes_value(true))
        .get_matches();
    let input = matches.value_of("input").unwrap_or("input");
    let data = fs::read_to_string(input).expect("unable to read file");
    let lines = data.split("\n").collect::<Vec<&str>>();

    let mut position = 0u64;
    let mut depth = 0u64;
    for line in lines {
        let l = line.split_whitespace().collect::<Vec<&str>>();
        let val = l[1].parse::<u64>().unwrap();
        match l[0] {
            "forward" => position += val,
            "down" => depth += val,
            "up" => depth -= val,
            _ => panic!("bad input"),
        }
        println!("{} {}", l[0], val);
    }

    println!("{}", position * depth);
}
