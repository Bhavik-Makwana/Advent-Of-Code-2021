use std::fmt::Error;

pub fn part_one(crabs: &Vec<i32>) -> Result<i32, Error> {
    let mut crabs2 = crabs.clone();
    let median = median(&mut crabs2);
    Ok(crabs.iter().map(|x| i32::abs(x - median)).sum())
}

fn median(numbers: &mut [i32]) -> i32 {
    numbers.sort();
    let mid = numbers.len() / 2;
    numbers[mid]
}

fn mean(numbers: &[i32]) -> i32 {
    numbers.iter().sum::<i32>() / numbers.len() as i32
}

// https://www.reddit.com/r/adventofcode/comments/rawxad/2021_day_7_part_2_i_wrote_a_paper_on_todays/
// referenced this paper someone else wrote up
pub fn part_two(crabs: &Vec<i32>) -> Result<i32, Error> {
    let mean = mean(crabs);
    Ok(i32::min(
        crabs.iter().map(|x| gauss(*x, mean)).sum(),
        crabs.iter().map(|x| gauss(*x, mean + 1)).sum(),
    ))
}

fn gauss(a: i32, b: i32) -> i32 {
    let n = i32::abs(a - b);
    (n * (n + 1)) / 2
}

#[cfg(test)]
mod tests {

    #[test]
    fn part_one_sample() {
        let vec = crate::readfile::fileio::read_list(String::from("input/test/day07.txt"));
        assert_eq!(crate::day07::part_one(&vec), Ok(37));
    }

    #[test]
    fn part_one_actual() {
        let vec = crate::readfile::fileio::read_list(String::from("input/day07.txt"));
        assert_eq!(crate::day07::part_one(&vec), Ok(345035));
    }

    #[test]
    fn part_two_sample() {
        let vec = crate::readfile::fileio::read_list(String::from("input/test/day07.txt"));
        assert_eq!(crate::day07::part_two(&vec), Ok(168));
    }

    #[test]
    fn part_two_actual() {
        let vec = crate::readfile::fileio::read_list(String::from("input/day07.txt"));
        assert_eq!(crate::day07::part_two(&vec), Ok(97038163));
    }
}
