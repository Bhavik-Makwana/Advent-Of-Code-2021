extern crate time;
use time::PreciseTime;

mod day06;
mod readfile;

pub use crate::readfile::fileio;

fn main() {
    let vec = fileio::read_list(String::from("input/day06.txt"));
    // let vec = fileio::read_file(String::from("input/day03.txt"));
    // let vec2 = fileio::read_file(String::from("input/day03.txt"));
    let ans = day06::part_one(&vec);
    println!("part one: {:#?}", ans.unwrap());
    let start = PreciseTime::now();
    let ans2 = day06::part_two(&vec).unwrap();
    let end = PreciseTime::now();
    println!("part two: {:#?}", ans2);
    println!("took: {} seconds", start.to(end));
}
