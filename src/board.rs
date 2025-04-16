use crate::interfaces::PointPlacement;
use anyhow::Context;
use std::ops::IndexMut;
use std::{
    iter::{zip, Iterator},
    ops::Index,
};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Board<T> {
    nrows: u16,
    ncolumns: u16,
    board: Vec<T>,
}

impl<T: std::marker::Copy> Board<T> {
    pub fn new<U: Into<T>>(nrows: u16, ncolumns: u16, initial_value: U) -> Self {
        let nelems = usize::from(nrows) * usize::from(ncolumns);
        let board = vec![initial_value.into(); nelems];
        Board {
            nrows,
            ncolumns,
            board,
        }
    }

    #[allow(dead_code)]
    pub fn new_with_values<Matrix, Row, U>(values: Matrix) -> anyhow::Result<Self>
    where
        U: Into<T> + std::marker::Copy,
        Matrix: AsRef<[Row]>,
        Row: AsRef<[U]>,
    {
        let board = values
            .as_ref()
            .iter()
            .flat_map(|row| row.as_ref().iter())
            .copied()
            .map(U::into)
            .collect::<Vec<T>>();
        let nrows = u16::try_from(values.as_ref().len())
            .context("Number of rows too big. Must fit in u16!")?;
        if nrows == 0 {
            anyhow::bail!("Number of rows must be greater than 0!");
        }
        let ncolumns = u16::try_from(board.len() / usize::from(nrows))
            .context("Number of columns too big. Must fit in u16!")?;
        Ok(Board::new_with_board(nrows, ncolumns, board))
    }

    #[allow(dead_code)]
    pub fn new_with_board(nrows: u16, ncolumns: u16, board: Vec<T>) -> Self {
        Board {
            nrows,
            ncolumns,
            board,
        }
    }

    pub fn get_number_of_rows(&self) -> u16 {
        self.nrows
    }

    pub fn get_number_of_columns(&self) -> u16 {
        self.ncolumns
    }

    pub fn iter_2d(&self) -> impl Iterator<Item = (PointPlacement, &T)> {
        self.board.iter().enumerate().map(|(index, val)| {
            let row = index / usize::from(self.ncolumns);
            let column = index % usize::from(self.ncolumns);
            let pp = PointPlacement { row, column };
            (pp, val)
        })
    }

    pub fn into_iter_2d(self) -> impl Iterator<Item = (PointPlacement, T)> {
        self.board.into_iter().enumerate().map(move |(index, val)| {
            let row = index / usize::from(self.ncolumns);
            let column = index % usize::from(self.ncolumns);
            let pp = PointPlacement { row, column };
            (pp, val)
        })
    }

    #[allow(dead_code)]
    pub fn joint_iter_2d<U: Copy>(
        self,
        board2: Board<U>,
    ) -> impl Iterator<Item = (PointPlacement, T, U)> {
        zip(self.into_iter_2d(), board2.into_iter_2d())
            .map(|(lhs, rhs)| (lhs.0, lhs.1, rhs.1))
    }

    fn to_index(&self, pp: PointPlacement) -> usize {
        pp.row * usize::from(self.ncolumns) + pp.column
    }
}

impl<T: std::marker::Copy> Index<PointPlacement> for Board<T> {
    type Output = T;

    fn index(&self, index: PointPlacement) -> &Self::Output {
        let index = self.to_index(index);
        &self.board[index]
    }
}

impl<T: std::marker::Copy> IndexMut<PointPlacement> for Board<T> {
    fn index_mut(&mut self, index: PointPlacement) -> &mut Self::Output {
        let index = self.to_index(index);
        &mut self.board[index]
    }
}

impl<T: std::marker::Copy + std::fmt::Display> std::fmt::Display for Board<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.nrows {
            for column in 0..(self.ncolumns - 1) {
                let pp = PointPlacement {
                    row: row.into(),
                    column: column.into(),
                };
                write!(f, "{}", self[pp])?;
            }

            let pp = PointPlacement {
                row: row.into(),
                column: (self.ncolumns - 1).into(),
            };
            writeln!(f, "{}", self[pp])?;
        }
        std::fmt::Result::Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interfaces::BoardStateEntry;
    use rstest::*;

    type GameState = Board<BoardStateEntry>;

    #[rstest]
    #[case(5, 5)]
    #[case(3, 3)]
    #[case(5, 10)]
    #[case(31, 11)]
    fn test_basic_board_workflow(#[case] nrows: u16, #[case] ncolumns: u16) {
        let mut board = GameState::new(nrows, ncolumns, None);
        let pp_min = PointPlacement { row: 0, column: 0 };
        let pp_max = PointPlacement {
            row: usize::from(nrows - 1),
            column: usize::from(ncolumns - 1),
        };
        assert!(board[pp_min].is_free());
        assert!(board[pp_max].is_free());
        board[pp_min] = Some(0).into();
        board[pp_max] = Some(1).into();
        assert!(board[pp_min].is_taken());
        assert!(board[pp_max].is_taken());
    }

    #[rstest]
    #[rstest]
    #[case(5, 5)]
    #[case(3, 3)]
    #[case(5, 10)]
    #[case(31, 11)]
    fn test_iter_2d(#[case] nrows: u16, #[case] ncolumns: u16) {
        let mut board = GameState::new(nrows, ncolumns, None);
        let nrows: usize = nrows.into();
        let ncolumns: usize = ncolumns.into();
        // [[0, 1, 2], [3, 4, 5], [6, 7, 8]]
        for row in 0..nrows {
            for column in 0..ncolumns {
                let val: usize = row * ncolumns + column;
                let pp = PointPlacement { row, column };
                board[pp] = Some(val).into();
            }
        }
        let mut result_iter = board.iter_2d();
        for row in 0..nrows {
            for column in 0..ncolumns {
                let pp = PointPlacement { row, column };
                let expected_val = Some(row * ncolumns + column).into();
                let result = result_iter.next().unwrap();
                assert_eq!(result, (pp, &expected_val));
            }
        }
        assert_eq!(result_iter.next(), None);
    }

    #[rstest]
    #[case(5, 5)]
    #[case(3, 3)]
    #[case(5, 10)]
    #[case(31, 11)]
    fn test_into_iter_2d(#[case] nrows: u16, #[case] ncolumns: u16) {
        let mut board = GameState::new(nrows, ncolumns, None);
        let nrows: usize = nrows.into();
        let ncolumns: usize = ncolumns.into();
        // [[0, 1, 2], [3, 4, 5], [6, 7, 8]]
        for row in 0..nrows {
            for column in 0..ncolumns {
                let val: usize = row * ncolumns + column;
                let pp = PointPlacement { row, column };
                board[pp] = Some(val).into();
            }
        }
        let mut result_iter = board.into_iter_2d();
        for row in 0..nrows {
            for column in 0..ncolumns {
                let pp = PointPlacement { row, column };
                let expected_val = Some(row * ncolumns + column).into();
                let result = result_iter.next().unwrap();
                assert_eq!(result, (pp, expected_val));
            }
        }
        assert_eq!(result_iter.next(), None);
    }

    #[rstest]
    #[case(5, 5)]
    #[case(3, 3)]
    #[case(5, 10)]
    #[case(31, 11)]
    fn test_joint_iter_2d_arrays(#[case] nrows: u16, #[case] ncolumns: u16) {
        let mut board = GameState::new(nrows, ncolumns, None);
        let mut board2 = GameState::new(nrows, ncolumns, None);
        let nrows: usize = nrows.into();
        let ncolumns: usize = ncolumns.into();
        // [[0, 1, 2], [3, 4, 5], [6, 7, 8]]
        for row in 0..nrows {
            for column in 0..ncolumns {
                let val = row * ncolumns + column;
                let val2 = nrows * ncolumns - 1 - val;
                let pp = PointPlacement { row, column };
                board[pp] = Some(val).into();
                board2[pp] = Some(val2).into();
            }
        }
        let mut result = board.joint_iter_2d(board2);

        for row in 0..nrows {
            for column in 0..ncolumns {
                let pp = PointPlacement { row, column };
                let expected_val = row * ncolumns + column;
                let expected_val2 = nrows * ncolumns - 1 - expected_val;

                assert_eq!(
                    result.next().unwrap(),
                    (pp, Some(expected_val).into(), Some(expected_val2).into())
                );
            }
        }
        assert_eq!(result.next(), None);
    }
}
