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
                2 => println!("is 1"),
                3 => println!("is 7"),
                4 => println!("is 4"),
                7 => println!("is 8"),
                5 => {
                    let mut total = 0;
                    let mut total2 = 0;
                    for i in number.chars() {
                        if matched[1].contains(i) {
                            total2 += 1;
                        }
                        if matched[4].contains(i) {
                            total += 1;
                        }
                    }
                    if total == 3 {
                        if total2 == 1 {
                            println!("5");
                            matched[5] = number.to_string();
                        } else if total2 == 2 {
                            println!("3");
                            matched[3] = number.to_string();
                        }
                    } else if total == 2 {
                        println!("2");
                        matched[2] = number.to_string();
                    }
                    // 5 takes 3 from 4
                    // 2 takes 2 from 4

                    // 5 take 1 from 1
                    // 3 take 2 from 1
                }
                6 => {
                    let mut total = 0;
                    let mut total2 = 0;
                    for i in number.chars() {
                        if matched[7].contains(i) {
                            total += 1;
                        }
                        if matched[4].contains(i) {
                            total2 += 1;
                        }
                    }

                    if total == 3 {
                        if total2 == 4 {
                            println!("9");
                            matched[9] = number.to_string();
                        } else {
                            println!("0");
                            matched[0] = number.to_string();
                        }
                    } else if total == 2 {
                        println!("6");
                        matched[6] = number.to_string();
                    }
                    // 9 takes 3 from 7
                    // 6 takes 2 from 7
                }
                _ => println!("malformed"),
            }
        }
        let mut out = "".to_string();
        for num in rhs.split(" ") {
            println!("{} {}", num, matched[5]);
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
        println!("output {}", out);
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
    // fn part_one_sample() {
    //     let vec = crate::readfile::fileio::read_file(String::from("input/test/day08.txt"));
    //     assert_eq!(crate::day08::part_one(&vec), Ok(26));
    // }

    // #[test]
    // fn part_one_actual() {
    //     let vec = crate::readfile::fileio::read_file(String::from("input/day08.txt"));
    //     assert_eq!(crate::day08::part_one(&vec), Ok(344));
    // }
    #[test]
    fn part_two_sample() {
        let vec = crate::readfile::fileio::read_file(String::from("input/test/day08.txt"));
        assert_eq!(crate::day08::part_two(&vec), Ok(61229));
    }

    // #[test]
    // fn part_two_actual() {
    //     let vec = crate::readfile::fileio::read_list(String::from("input/day08.txt"));
    //     assert_eq!(crate::day08::part_two(&vec), Ok(97038163));
    // }
}
