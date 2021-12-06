use clap::{App, Arg};
use std::fs;

fn main() {
    let matches = App::new("aoc-01-02")
        .arg(Arg::with_name("input").short("i").takes_value(true))
        .get_matches();
    let input = matches.value_of("input").unwrap_or("input");
    let data = fs::read_to_string(input).expect("unable to read file");
    let lines = data.split_whitespace().collect::<Vec<&str>>();

    let mut count = 0;
    let mut last = lines[0].parse::<u64>().unwrap();
    for line in lines {
        let val = line.parse::<u64>().unwrap();
        if val > last {
            count = count + 1;
        }
        last = val;
    }

    println!("{}", count);
}
