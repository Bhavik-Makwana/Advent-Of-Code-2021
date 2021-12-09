use ndarray::prelude::*;
use std::collections::HashSet;
use std::fmt::Error;
use std::iter::FromIterator;

pub fn part_one(readings: &Vec<Vec<char>>) -> Result<isize, Error> {
    let mut gamma = Vec::new();
    let mut epsilon = Vec::new();
    for i in 0..readings[0].len() {
        let mut column: i64 = readings
            .iter() // iterate over rows
            .map(|x| x[i].to_digit(10).unwrap() as i64) // get the icolumn-th element from each row
            .collect::<Vec<i64>>()
            .iter()
            .map(|x| if *x == 0 { -1 } else { *x })
            .sum();
        if column > 0 {
            gamma.push(1);
            epsilon.push(0);
        } else {
            gamma.push(0);
            epsilon.push(1);
        }
    }
    let g: String = gamma.iter().map(|&id| id.to_string()).collect();
    let e: String = epsilon.iter().map(|&id| id.to_string()).collect();
    let a = isize::from_str_radix(&g, 2).unwrap();
    let b = isize::from_str_radix(&e, 2).unwrap();
    Ok(a * b)
}

pub fn part_two(readings: &Vec<String>) -> Result<i64, Error> {
    let MSB: usize = readings[0].len();
    let mut o2: Vec<i64> = readings
        .iter()
        .map(|x| i64::from_str_radix(&x, 2).unwrap())
        .collect();

    for bit in (0..MSB).rev() {
        let (ones, zeros): (Vec<i64>, Vec<i64>) =
            o2.iter().partition(|&o| o | 2i64.pow(bit as u32) == *o);
        if ones.len() >= zeros.len() {
            o2 = ones;
        } else {
            o2 = zeros;
        }
        if o2.len() == 1 {
            break;
        }
    }
    println!("O2: {:?}", o2[0]);

    let mut co2: Vec<i64> = readings
        .iter()
        .map(|x| i64::from_str_radix(&x, 2).unwrap())
        .collect();
    for bit in (0..MSB).rev() {
        let (ones, zeros): (Vec<i64>, Vec<i64>) =
            co2.iter().partition(|&co| co | 2i64.pow(bit as u32) == *co);
        if ones.len() >= zeros.len() {
            co2 = zeros;
        } else {
            co2 = ones;
        }
        if co2.len() == 1 {
            break;
        }
    }

    Ok(o2[0] * co2[0] + 1)
}

#[cfg(test)]
mod tests {

    #[test]
    fn part_one_sample() {
        let vec = crate::readfile::fileio::read_file_2d(String::from("input/test/day03.txt"));
        assert_eq!(crate::day03::part_one(&vec), Ok(198));
    }

    // #[test]
    // fn part_one_actual() {
    //     let vec = crate::readfile::fileio::read_file_int(String::from("input/day01.txt"));
    //     assert_eq!(crate::day01::part_one(&vec), Ok(1154));
    // }

    #[test]
    fn part_two_sample() {
        let vec = crate::readfile::fileio::read_file(String::from("input/test/day03.txt"));
        assert_eq!(crate::day03::part_two(&vec), Ok(230));
    }

    // #[test]
    // fn part_two_actual() {
    //     let vec = crate::readfile::fileio::read_file_int(String::from("input/day01.txt"));
    //     assert_eq!(crate::day01::part_two(&vec), Ok(1127));
    // }
}
