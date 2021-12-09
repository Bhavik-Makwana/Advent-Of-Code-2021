pub mod board;
use board::Board;

use std::fmt::Error;

pub fn part_one(bingo_input: &Vec<Vec<String>>) -> Result<i32, Error> {
    let rollcall: Vec<i32> = bingo_input.into_iter().next().unwrap()[0]
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect();

    let mut bingo_boards: Vec<Board> = bingo_input
        .into_iter()
        .skip(1)
        .map(|board| store_board(board))
        .collect();

    for number in rollcall.iter() {
        bingo_boards = bingo_boards
            .iter()
            .map(|board| board.mark_board(number))
            .collect::<Vec<Board>>();

        if bingo_boards
            .iter()
            .filter(|board| board.check_win())
            .collect::<Vec<&Board>>()
            .len()
            == 1
        {
            let winning_board = bingo_boards
                .iter()
                .filter(|board| board.check_win())
                .collect::<Vec<&Board>>()[0];

            println!("{:?}", winning_board);
            return Ok(winning_board
                .board
                .iter()
                .flatten()
                .filter(|x| **x != -1)
                .sum::<i32>()
                * number);
        }
    }
    Err(Error)
}

pub fn part_two(bingo_input: &Vec<Vec<String>>) -> Result<i32, Error> {
    let rollcall: Vec<i32> = bingo_input.into_iter().next().unwrap()[0]
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect();
    let mut bingo_boards: Vec<Board> = bingo_input
        .into_iter()
        .skip(1)
        .map(|board| store_board(board))
        .collect();
    for (i, number) in rollcall.iter().enumerate() {
        bingo_boards = bingo_boards
            .iter()
            .map(|board| board.mark_board(number))
            .collect::<Vec<Board>>();
        if bingo_boards
            .iter()
            .filter(|board| !board.check_win())
            .collect::<Vec<&Board>>()
            .len()
            == 1
        {
            let losing_board = bingo_boards
                .iter()
                .filter(|board| !board.check_win())
                .collect::<Vec<&Board>>()[0];
            for j in i..rollcall.len() - 1 {
                let losing_board = &losing_board.mark_board(&rollcall[j + 1]);
                if losing_board.check_win() {
                    let t = losing_board
                        .board
                        .iter()
                        .flatten()
                        .filter(|x| **x != -1)
                        .sum::<i32>();
                    return Ok(t * &rollcall[i + 1]);
                }
            }
        }
    }
    Err(Error)
}

fn store_board(board_in: &Vec<String>) -> Board {
    let board: Vec<Vec<i32>> = board_in
        .into_iter()
        .map(|row| {
            row.split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();
    Board { board }
}

#[cfg(test)]
mod tests {

    #[test]
    fn part_one_sample() {
        let vec = crate::readfile::fileio::read_file_batch(String::from("input/test/day04.txt"));
        assert_eq!(crate::day04::part_one(&vec), Ok(4512));
    }

    #[test]
    fn part_one_actual() {
        let vec = crate::readfile::fileio::read_file_batch(String::from("input/day04.txt"));
        assert_eq!(crate::day04::part_one(&vec), Ok(67716));
    }

    #[test]
    fn part_two_sample() {
        let vec = crate::readfile::fileio::read_file_batch(String::from("input/test/day04.txt"));
        assert_eq!(crate::day04::part_two(&vec), Ok(1924));
    }

    #[test]
    fn part_two_actual() {
        let vec = crate::readfile::fileio::read_file_batch(String::from("input/day04.txt"));
        assert_eq!(crate::day04::part_two(&vec), Ok(1830));
    }
}
