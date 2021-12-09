extern crate time;
use time::PreciseTime;

mod day04;
mod readfile;

pub use crate::readfile::fileio;

fn main() {
    let vec = fileio::read_file_batch(String::from("input/day04.txt"));
    // let vec = fileio::read_file(String::from("input/day03.txt"));
    // let vec2 = fileio::read_file(String::from("input/day03.txt"));
    let ans = day04::part_one(&vec);
    println!("part one: {:#?}", ans.unwrap());
    let start = PreciseTime::now();
    let ans2 = day04::part_two(&vec).unwrap();
    let end = PreciseTime::now();
    println!("part two: {:#?}", ans2);
    println!("took: {} seconds", start.to(end));
}
