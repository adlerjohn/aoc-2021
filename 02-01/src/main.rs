use clap::{App, Arg};
use std::fs;

fn main() {
    let matches = App::new("aoc-02-01")
        .arg(Arg::with_name("input").short("i").takes_value(true))
        .get_matches();
    let input = matches.value_of("input").unwrap_or("input");
    let data = fs::read_to_string(input).expect("unable to read file");
    let lines = data.split_whitespace().collect::<Vec<&str>>();

    let mut count = 0u64;
    let mut last = u64::MAX;
    for (i, _) in lines.iter().enumerate() {
        // Break if there are insufficient elements left
        if i > lines.len() - 3 {
            break;
        }
        let mut sum = 0u64;
        for j in 0..3 {
            sum = sum + lines[i + j].parse::<u64>().unwrap();
        }
        if sum > last {
            count = count + 1;
        }
        last = sum;
    }

    println!("{}", count);
}
