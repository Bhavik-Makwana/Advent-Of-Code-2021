use itertools::Itertools;
use std::fmt::Error;

pub fn part_one(times: &Vec<String>) -> Result<i32, Error> {
    Ok(times
        .iter()
        .map(|time| count(time.split("|").skip(1).next().unwrap()))
        .sum())
}

fn count(rhs: &str) -> i32 {
    rhs.split(" ")
        .map(|x| x.len())
        .filter(|x| *x == 2 || *x == 3 || *x == 4 || *x == 7)
        .count() as i32
}

pub fn part_two(times: &Vec<String>) -> Result<i32, Error> {
    let mut total = 0;
    for time in times.iter() {
        let mut times_iter = time.split("|");
        let lhs = times_iter.next().unwrap();
        let rhs = times_iter.next().unwrap();
        let mut matched = vec!["".to_string(); 10];
        matched[1] = lhs.split(" ").filter(|x| x.len() == 2).collect();
        matched[4] = lhs.split(" ").filter(|x| x.len() == 4).collect();
        matched[7] = lhs.split(" ").filter(|x| x.len() == 3).collect();
        matched[8] = lhs.split(" ").filter(|x| x.len() == 7).collect();

        for number in lhs.split(" ") {
            match number.len() {
                5 => {
                    let matches_with_one = number
                        .chars()
                        .filter(|i| matched[1].contains(*i))
                        .collect::<String>()
                        .len();
                    let matches_with_four = number
                        .chars()
                        .filter(|i| matched[4].contains(*i))
                        .collect::<String>()
                        .len();

                    if matches_with_four == 3 {
                        // 5 takes 3 from 4
                        if matches_with_one == 1 {
                            // 5 take 1 from 1
                            matched[5] = number.to_string();
                        } else if matches_with_one == 2 {
                            // 3 take 2 from 1
                            matched[3] = number.to_string();
                        }
                    } else if matches_with_four == 2 {
                        // 2 takes 2 from 4
                        matched[2] = number.to_string();
                    }
                }
                6 => {
                    let matches_with_seven = number
                        .chars()
                        .filter(|i| matched[7].contains(*i))
                        .collect::<String>()
                        .len();
                    let matches_with_four = number
                        .chars()
                        .filter(|i| matched[4].contains(*i))
                        .collect::<String>()
                        .len();

                    if matches_with_seven == 3 {
                        // 9 takes 3 from 7
                        if matches_with_four == 4 {
                            // 9 takes 4 from 4
                            matched[9] = number.to_string();
                        } else {
                            // 0 takes 3 from 4
                            matched[0] = number.to_string();
                        }
                    } else if matches_with_seven == 2 {
                        // 6 takes 2 from 7
                        matched[6] = number.to_string();
                    }
                }
                _ => println!("malformed"),
            }
        }
        let mut out = "".to_string();
        for num in rhs.split(" ") {
            let x = sort_str(&num.to_string());
            match x {
                _ if x == sort_str(&matched[1]) => out += "1",
                _ if x == sort_str(&matched[2]) => out += "2",
                _ if x == sort_str(&matched[3]) => out += "3",
                _ if x == sort_str(&matched[4]) => out += "4",
                _ if x == sort_str(&matched[5]) => out += "5",
                _ if x == sort_str(&matched[6]) => out += "6",
                _ if x == sort_str(&matched[7]) => out += "7",
                _ if x == sort_str(&matched[8]) => out += "8",
                _ if x == sort_str(&matched[9]) => out += "9",
                _ if x == sort_str(&matched[0]) => out += "0",
                _ => (),
            }
        }
        total += out.parse::<i32>().unwrap();
    }
    Ok(total)
}

fn sort_str(s: &String) -> String {
    s.chars().sorted().rev().collect::<String>()
}

#[cfg(test)]
mod tests {

    #[test]
    fn part_one_sample() {
        let vec = crate::readfile::fileio::read_file(String::from("input/test/day08.txt"));
        assert_eq!(crate::day08::part_one(&vec), Ok(26));
    }

    #[test]
    fn part_one_actual() {
        let vec = crate::readfile::fileio::read_file(String::from("input/day08.txt"));
        assert_eq!(crate::day08::part_one(&vec), Ok(344));
    }
    #[test]
    fn part_two_sample() {
        let vec = crate::readfile::fileio::read_file(String::from("input/test/day08.txt"));
        assert_eq!(crate::day08::part_two(&vec), Ok(61229));
    }

    #[test]
    fn part_two_actual() {
        let vec = crate::readfile::fileio::read_file(String::from("input/day08.txt"));
        assert_eq!(crate::day08::part_two(&vec), Ok(1048410));
    }
}
