extern crate time;
use time::PreciseTime;

mod day02;
mod readfile;

pub use crate::readfile::fileio;

fn main() {
    // let vec = fileio::read_file(String::from("input/test/day02.txt"));
    let vec = fileio::read_file(String::from("input/day02.txt"));

    let ans = day02::part_one(&vec);
    println!("part one: {:#?}", ans.unwrap());
    let start = PreciseTime::now();
    let ans2 = day02::part_two(&vec).unwrap();
    let end = PreciseTime::now();
    println!("part two: {:#?}", ans2);
    println!("took: {} seconds", start.to(end));
}
