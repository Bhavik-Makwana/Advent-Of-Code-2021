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
    let width = heatmap.len();
    let height = heatmap[0].len();
    for l in 0..8 {
        let ti = (i as i32 + moves[l].0) as usize;
        let tj = (j as i32 + moves[l].1) as usize;
        if ti >= width || tj >= height {
            // dont need to comp against 0 as using unsigned int
            continue;
        }
        if heatmap[i][j] >= heatmap[ti][tj] {
            return false;
        }
    }
    true
}

pub fn part_two(heatmap: &Vec<Vec<i32>>) -> Result<i32, Error> {
    let mut results = Vec::new();
    for (i, x) in heatmap.iter().enumerate() {
        for (j, _) in x.iter().enumerate() {
            if is_low_point(heatmap, i, j) {
                let b = basin_size(heatmap, i, j);
                results.push(b);
            }
        }
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
    bfs(heatmap, &mut visited, i, j)
}

fn bfs(_heatmap: &Vec<Vec<i32>>, visited: &mut Vec<Vec<bool>>, i: usize, j: usize) -> i32 {
    let mut q: Queue<(usize, usize)> = queue![];
    let mut total = 0;
    q.add((i, j)).expect("bad input to queue");
    visited[i][j] = true;

    let row_dir: Vec<i32> = vec![-1, 0, 1, 0];
    let col_dir: Vec<i32> = vec![0, 1, 0, -1];

    while !(q.size() == 0) {
        let cell = q.peek().unwrap();
        // println!("{} ", heatmap[cell.0][cell.1]);
        total += 1;
        q.remove().expect("queue empty");

        for i in 0..4 {
            let adjx = (cell.0 as i32 + row_dir[i]) as usize;
            let adjy = (cell.1 as i32 + col_dir[i]) as usize;

            if is_valid(visited, adjx, adjy) {
                q.add((adjx, adjy)).expect("bad input to queue");
                visited[adjx][adjy] = true;
            }
        }
    }
    total
}

fn is_valid(visited: &Vec<Vec<bool>>, i: usize, j: usize) -> bool {
    let width = visited.len();
    let height = visited[0].len();
    if i >= width || j >= height {
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

    #[test]
    fn part_two_actual() {
        let vec = crate::readfile::fileio::read_file_2d_i32(String::from("input/day09.txt"));
        assert_eq!(crate::day09::part_two(&vec), Ok(1148965));
    }
}
