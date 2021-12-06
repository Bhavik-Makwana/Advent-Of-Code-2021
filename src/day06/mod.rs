use std::fmt::Error;

pub fn part_one(lanternfish: &Vec<i32>) -> Result<u64, Error> {
    Ok(calculator(lanternfish, 80))
}

pub fn part_two(lanternfish: &Vec<i32>) -> Result<u64, Error> {
    Ok(calculator(lanternfish, 256))
}

fn calculator(lanternfish: &Vec<i32>, days: usize) -> u64 {
    let mut cache: Vec<u64> = vec![0; 9];
    for i in lanternfish.iter() {
        cache[*i as usize] += 1;
    }

    for _ in 0..days {
        let mut inner = vec![0; 9];
        for bucket in (1..9).rev() {
            inner[bucket - 1] += cache[bucket];
        }
        inner[8] += cache[0];
        inner[6] += cache[0];
        cache = inner;
    }
    cache.iter().sum()
}

#[cfg(test)]
mod tests {

    #[test]
    fn part_one_sample() {
        let vec = crate::readfile::fileio::read_list(String::from("input/test/day06.txt"));
        assert_eq!(crate::day06::part_one(&vec), Ok(5934));
    }

    #[test]
    fn part_one_actual() {
        let vec = crate::readfile::fileio::read_list(String::from("input/day06.txt"));
        assert_eq!(crate::day06::part_one(&vec), Ok(394994));
    }

    #[test]
    fn part_two_sample() {
        let vec = crate::readfile::fileio::read_list(String::from("input/test/day06.txt"));
        assert_eq!(crate::day06::part_two(&vec), Ok(26984457539));
    }

    #[test]
    fn part_two_actual() {
        let vec = crate::readfile::fileio::read_list(String::from("input/day06.txt"));
        assert_eq!(crate::day06::part_two(&vec), Ok(1765974267455));
    }
}
