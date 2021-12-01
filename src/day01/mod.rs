use std::collections::HashSet;
use std::fmt::Error;
use std::iter::FromIterator;

pub fn part_one(readings: &Vec<i32>) -> Result<usize, Error> {
    Ok(readings.windows(2).filter(|&reading| reading[1] > reading[0]).count())
}

pub fn part_two(readings: &Vec<i32>) -> Result<usize, Error> {
    Ok(readings.windows(4).filter(|&window| window[0..3].iter().sum::<i32>() < window[1..4].iter().sum::<i32>()).count())
}

#[cfg(test)]
mod tests {

    #[test]
    fn part_one_sample() {
        let vec = crate::readfile::fileio::read_file_int(String::from("input/test/day01.txt"));
        assert_eq!(crate::day01::part_one(&vec), Ok(7));
    }

    #[test]
    fn part_one_actual() {
        let vec = crate::readfile::fileio::read_file_int(String::from("input/day01.txt"));
        assert_eq!(crate::day01::part_one(&vec), Ok(1154));
    }

    #[test]
    fn part_two_sample() {
        let vec = crate::readfile::fileio::read_file_int(String::from("input/test/day01.txt"));
        assert_eq!(crate::day01::part_two(&vec), Ok(5));
    }

    #[test]
    fn part_two_actual() {
        let vec = crate::readfile::fileio::read_file_int(String::from("input/day01.txt"));
        assert_eq!(crate::day01::part_two(&vec), Ok(1127));
    }
}
