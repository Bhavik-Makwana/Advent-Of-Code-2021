extern crate time;
use time::PreciseTime;

mod day01;
mod readfile;

pub use crate::readfile::fileio;

fn main() {
    // let vec = fileio::read_file_int(String::from("input/test/day01.txt"));
    let vec = fileio::read_file_int(String::from("input/day1.txt"));

    let ans = day01::part_one(&vec);
    println!("part one: {:#?}", ans.unwrap());
    let start = PreciseTime::now();
    let ans2 = day01::part_two(&vec).unwrap();
    let end = PreciseTime::now();
    println!("part two: {:#?}", ans2);
    println!("took: {} seconds", start.to(end));
}
