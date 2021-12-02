use std::fmt::Error;

pub fn part_one(readings: &Vec<String>) -> Result<i32, Error> {
    let position = readings
        .iter()
        .fold((0, 0), |(mut horizontal, mut depth), reading| {
            let x: Vec<&str> = reading.split(" ").collect();
            match x[0] {
                "forward" => horizontal += x[1].parse::<i32>().unwrap(),
                "up" => depth -= x[1].parse::<i32>().unwrap(),
                "down" => depth += x[1].parse::<i32>().unwrap(),
                _ => (),
            }
            (horizontal, depth)
        });
    Ok(position.0 * position.1)
}

pub fn part_two(readings: &Vec<String>) -> Result<i32, Error> {
    let position = readings.iter().fold(
        (0, 0, 0),
        |(mut horizontal, mut depth, mut aim), reading| {
            let x: Vec<&str> = reading.split(" ").collect();
            match x[0] {
                "forward" => {
                    horizontal += x[1].parse::<i32>().unwrap();
                    depth += x[1].parse::<i32>().unwrap() * aim;
                }
                "up" => aim -= x[1].parse::<i32>().unwrap(),
                "down" => aim += x[1].parse::<i32>().unwrap(),
                _ => (),
            }
            (horizontal, depth, aim)
        },
    );
    Ok(position.0 * position.1)
}

#[cfg(test)]
mod tests {

    #[test]
    fn part_one_sample() {
        let vec = crate::readfile::fileio::read_file(String::from("input/test/day02.txt"));
        assert_eq!(crate::day02::part_one(&vec), Ok(150));
    }

    #[test]
    fn part_one_actual() {
        let vec = crate::readfile::fileio::read_file(String::from("input/day02.txt"));
        assert_eq!(crate::day02::part_one(&vec), Ok(1459206));
    }

    #[test]
    fn part_two_sample() {
        let vec = crate::readfile::fileio::read_file(String::from("input/test/day02.txt"));
        assert_eq!(crate::day02::part_two(&vec), Ok(900));
    }

    #[test]
    fn part_two_actual() {
        let vec = crate::readfile::fileio::read_file(String::from("input/day02.txt"));
        assert_eq!(crate::day02::part_two(&vec), Ok(1320534480));
    }
}
