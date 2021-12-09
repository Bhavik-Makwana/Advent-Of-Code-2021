use itertools::Itertools;
use std::fmt::Error;
extern crate queues;
use queues::*;
use std::cmp::Reverse;

pub fn part_one(heatmap: &Vec<Vec<i32>>) -> Result<i32, Error> {
    let mut total = 0;
    for (i, x) in heatmap.iter().enumerate() {
        for (j, e) in x.iter().enumerate() {
            if is_low_point(heatmap, i, j) {
                total += e + 1;
            }
        }
    }
    Ok(total)
}

fn is_low_point(heatmap: &Vec<Vec<i32>>, i: usize, j: usize) -> bool {
    let moves: [(i32, i32); 8] = [
        (-1, 0),
        (-1, -1),
        (0, -1),
        (1, -1),
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
    ];
    let WIDTH = heatmap.len();
    let HEIGHT = heatmap[0].len();
    for l in 0..8 {
        let ti = (i as i32 + moves[l].0) as usize;
        let tj = (j as i32 + moves[l].1) as usize;
        if ti < 0 || ti >= WIDTH || tj < 0 || tj >= HEIGHT {
            continue;
        }
        if heatmap[i][j] >= heatmap[ti][tj] {
            return false;
        }
    }
    true
}

pub fn part_two(heatmap: &Vec<Vec<i32>>) -> Result<i32, Error> {
    let mut total = 0;
    let mut results = Vec::new();
    for (i, x) in heatmap.iter().enumerate() {
        for (j, e) in x.iter().enumerate() {
            if is_low_point(heatmap, i, j) {
                let b = basin_size(heatmap, i, j);
                println!("Basin size: {}", b);
                results.push(b);
            }
        }
        // println!("");
    }
    results.sort_by_key(|w| Reverse(*w));
    Ok(results[0] * results[1] * results[2])
}

fn basin_size(heatmap: &Vec<Vec<i32>>, i: usize, j: usize) -> i32 {
    let mut visited: Vec<Vec<bool>> = vec![vec![false; heatmap[0].len()]; heatmap.len()];
    for (i, x) in heatmap.iter().enumerate() {
        for (j, e) in x.iter().enumerate() {
            if *e == 9 {
                visited[i][j] = true;
            }
        }
    }
    // println!("{}", s);
    bfs(heatmap, &mut visited, i, j)
}

fn bfs(heatmap: &Vec<Vec<i32>>, visited: &mut Vec<Vec<bool>>, i: usize, j: usize) -> i32 {
    let mut q: Queue<(usize, usize)> = queue![];
    let mut total = 0;
    q.add((i, j));
    visited[i][j] = true;

    let dRow: Vec<i32> = vec![-1, 0, 1, 0];
    let dCol: Vec<i32> = vec![0, 1, 0, -1];

    while !(q.size() == 0) {
        let cell = q.peek().unwrap();
        println!("{} ", heatmap[cell.0][cell.1]);
        total += 1;
        q.remove();

        for i in 0..4 {
            let adjx = (cell.0 as i32 + dRow[i]) as usize;
            let adjy = (cell.1 as i32 + dCol[i]) as usize;

            if is_valid(visited, adjx, adjy) {
                q.add((adjx, adjy));
                visited[adjx][adjy] = true;
            }
        }
    }
    total
}

fn is_valid(visited: &Vec<Vec<bool>>, i: usize, j: usize) -> bool {
    let WIDTH = visited.len();
    let HEIGHT = visited[0].len();
    if i < 0 || j < 0 || i >= WIDTH || j >= HEIGHT {
        return false;
    }

    if visited[i][j] {
        return false;
    }

    true
}

#[cfg(test)]
mod tests {

    #[test]
    fn part_one_sample() {
        let vec = crate::readfile::fileio::read_file_2d_i32(String::from("input/test/day09.txt"));
        assert_eq!(crate::day09::part_one(&vec), Ok(15));
    }

    #[test]
    fn part_one_actual() {
        let vec = crate::readfile::fileio::read_file_2d_i32(String::from("input/day09.txt"));
        assert_eq!(crate::day09::part_one(&vec), Ok(417));
    }

    #[test]
    fn part_two_sample() {
        let vec = crate::readfile::fileio::read_file_2d_i32(String::from("input/test/day09.txt"));
        assert_eq!(crate::day09::part_two(&vec), Ok(1134));
    }

    // #[test]
    // fn part_two_actual() {
    //     let vec = crate::readfile::fileio::read_file(String::from("input/day09.txt"));
    //     assert_eq!(crate::day09::part_two(&vec), Ok(1048410));
    // }
}
