#[derive(Debug)]
pub struct Board {
    pub board: Vec<Vec<i32>>,
}

impl Board {
    pub fn check_win(&self) -> bool {
        if self
            .board
            .iter()
            .filter(|row| {
                row.iter()
                    .filter(|x| **x == -1)
                    .collect::<Vec<&i32>>()
                    .len()
                    == 5
            })
            .collect::<Vec<&Vec<i32>>>()
            .len()
            >= 1
        {
            return true;
        }
        self.transpose_check()
    }

    fn transpose_check(&self) -> bool {
        assert!(!self.board.is_empty());
        let len = self.board[0].len();
        let mut iters: Vec<_> = self.board.iter().map(|n| n.into_iter()).collect();
        let vec: Vec<Vec<&i32>> = (0..len)
            .map(|_| {
                iters
                    .iter_mut()
                    .map(|n| n.next().unwrap())
                    .collect::<Vec<&i32>>()
            })
            .collect();
        if vec
            .iter()
            .filter(|row| {
                row.iter()
                    .filter(|x| ***x == -1)
                    .collect::<Vec<&&i32>>()
                    .len()
                    == 5
            })
            .collect::<Vec<&Vec<&i32>>>()
            .len()
            >= 1
        {
            return true;
        }
        false
    }

    pub fn mark_board(&self, number: &i32) -> Board {
        let ret_board = self
            .board
            .iter()
            .map(|row| {
                row.iter()
                    .map(|x| if x == number { -1 } else { *x })
                    .collect()
            })
            .collect();

        Board { board: ret_board }
    }
}
