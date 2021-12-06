use clap::{App, Arg};
use std::fs;

fn main() {
    let matches = App::new("aoc-03-01")
        .arg(Arg::with_name("input").short("i").takes_value(true))
        .get_matches();
    let input = matches.value_of("input").unwrap_or("input");
    let data = fs::read_to_string(input).expect("unable to read file");
    let lines = data.split("\n").collect::<Vec<&str>>();

    let mut gamma = 0u64;
    let mut epsilon = 0u64;
    // Note: input is 12 bits
    for i in 0..12 {
        let mut count_ones = 0u64;
        for line in &lines {
            let bit = line.chars().nth(i).unwrap();
            match bit {
                '0' => count_ones += 0,
                '1' => count_ones += 1,
                _ => panic!("bad input"),
            }
        }
        let count_zeroes = lines.len() as u64 - count_ones;
        // TODO what if they're equal? Doesn't happen in the input.
        // Only add non-zero bit
        if count_zeroes > count_ones {
            epsilon = epsilon + (1 << (11 - i));
        } else {
            gamma = gamma + (1 << (11 - i));
        }
    }

    println!("{}", gamma * epsilon);
}
