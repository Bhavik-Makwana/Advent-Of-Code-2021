use std::fmt::Error;

pub fn part_one(lanternfish: &Vec<i32>) -> Result<usize, Error> {
    let mut lanternfish_mut: Vec<i32> = lanternfish.clone();
    for i in 0..80 {
        let babies = lanternfish_mut.iter().filter(|x| **x == 0).count();
        lanternfish_mut = lanternfish_mut
            .iter()
            .map(|x| if x - 1 == -1 { 6 } else { x - 1 })
            .collect();
        for j in 0..babies {
            lanternfish_mut.push(8);
        }
    }
    Ok(lanternfish_mut.len())
}

pub fn part_two(lanternfish: &Vec<i32>) -> Result<u64, Error> {
    let mut cache: Vec<u64> = vec![0; 9];
    for i in lanternfish.iter() {
        cache[*i as usize] += 1;
    }

    for _day in 0..256 {
        let mut inner = vec![0; 9];

        for i in (0..9).rev() {
            match i {
                0 => {
                    inner[8] += cache[0];
                    inner[6] += cache[0];
                }
                _ => {
                    inner[i - 1] += cache[i];
                }
            }
        }
        cache = inner;
    }
    Ok(cache.iter().sum())
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
