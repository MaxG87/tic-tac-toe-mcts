use crate::interfaces::{BoardStateEntry, PointPlacement};
use anyhow::Context;
use std::ops::IndexMut;
use std::{
    iter::{zip, Iterator},
    ops::Index,
};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Board {
    nrows: u16,
    ncolumns: u16,
    board: Vec<BoardStateEntry>,
}

#[allow(dead_code)]
impl Board {
    pub fn new(nrows: u16, ncolumns: u16) -> Self {
        let nelems = usize::from(nrows) * usize::from(ncolumns);
        let board = vec![None; nelems];
        Board {
            nrows,
            ncolumns,
            board,
        }
    }

    pub fn new_with_values<Matrix, Row>(values: Matrix) -> anyhow::Result<Self>
    where
        Matrix: AsRef<[Row]>,
        Row: AsRef<[BoardStateEntry]>,
    {
        let board = values
            .as_ref()
            .iter()
            .flat_map(|row| row.as_ref().iter())
            .copied()
            .collect::<Vec<_>>();
        let nrows = u16::try_from(values.as_ref().len())
            .context("Number of rows too big. Must fit in u16!")?;
        if nrows == 0 {
            anyhow::bail!("Number of rows must be greater than 0!");
        }
        let ncolumns = u16::try_from(board.len() / usize::from(nrows))
            .context("Number of columns too big. Must fit in u16!")?;
        Ok(Board::new_with_board(nrows, ncolumns, board))
    }

    pub fn new_with_board(
        nrows: u16,
        ncolumns: u16,
        board: Vec<BoardStateEntry>,
    ) -> Self {
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

    pub fn has_placement_at(&self, pp: PointPlacement) -> bool {
        let index = self.to_index(pp);
        self.board[index].is_some()
    }

    pub fn iter_2d(&self) -> impl Iterator<Item = (PointPlacement, &BoardStateEntry)> {
        self.board.iter().enumerate().map(|(index, val)| {
            let row = index / usize::from(self.ncolumns);
            let column = index % usize::from(self.ncolumns);
            let pp = PointPlacement { row, column };
            (pp, val)
        })
    }

    pub fn into_iter_2d(
        self,
    ) -> impl Iterator<Item = (PointPlacement, BoardStateEntry)> {
        self.board.into_iter().enumerate().map(move |(index, val)| {
            let row = index / usize::from(self.ncolumns);
            let column = index % usize::from(self.ncolumns);
            let pp = PointPlacement { row, column };
            (pp, val)
        })
    }

    pub fn joint_iter_2d(
        self,
        board2: Board,
    ) -> impl Iterator<Item = (PointPlacement, BoardStateEntry, BoardStateEntry)> {
        zip(self.into_iter_2d(), board2.into_iter_2d())
            .map(|(lhs, rhs)| (lhs.0, lhs.1, rhs.1))
    }

    fn to_index(&self, pp: PointPlacement) -> usize {
        pp.row * usize::from(self.ncolumns) + pp.column
    }
}

impl Index<PointPlacement> for Board {
    type Output = BoardStateEntry;

    fn index(&self, index: PointPlacement) -> &Self::Output {
        let index = self.to_index(index);
        &self.board[index]
    }
}

impl IndexMut<PointPlacement> for Board {
    fn index_mut(&mut self, index: PointPlacement) -> &mut Self::Output {
        let index = self.to_index(index);
        &mut self.board[index]
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.nrows {
            for column in 0..(self.ncolumns - 1) {
                let pp = PointPlacement {
                    row: row.into(),
                    column: column.into(),
                };
                write!(
                    f,
                    "{}",
                    match self[pp] {
                        Some(id) => format!("{id}"),
                        None => ".".to_string(),
                    }
                )?;
            }

            let pp = PointPlacement {
                row: row.into(),
                column: (self.ncolumns - 1).into(),
            };
            writeln!(
                f,
                "{}",
                match self[pp] {
                    Some(id) => format!("{id}"),
                    None => ".".to_string(),
                }
            )?;
        }
        std::fmt::Result::Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(5, 5)]
    #[case(3, 3)]
    #[case(5, 10)]
    #[case(31, 11)]
    fn test_basic_board_workflow(#[case] nrows: u16, #[case] ncolumns: u16) {
        let mut board = Board::new(nrows, ncolumns);
        let pp_min = PointPlacement { row: 0, column: 0 };
        let pp_max = PointPlacement {
            row: usize::from(nrows - 1),
            column: usize::from(ncolumns - 1),
        };
        assert!(!board.has_placement_at(pp_min));
        assert!(!board.has_placement_at(pp_max));
        board[pp_min] = Some(0);
        board[pp_max] = Some(1);
        assert!(board.has_placement_at(pp_min));
        assert!(board.has_placement_at(pp_max));
    }

    #[rstest]
    #[rstest]
    #[case(5, 5)]
    #[case(3, 3)]
    #[case(5, 10)]
    #[case(31, 11)]
    fn test_iter_2d(#[case] nrows: u16, #[case] ncolumns: u16) {
        let mut board = Board::new(nrows, ncolumns);
        let nrows: usize = nrows.into();
        let ncolumns: usize = ncolumns.into();
        // [[0, 1, 2], [3, 4, 5], [6, 7, 8]]
        for row in 0..nrows {
            for column in 0..ncolumns {
                let val: usize = row * ncolumns + column;
                let pp = PointPlacement { row, column };
                board[pp] = Some(val);
            }
        }
        let mut result_iter = board.iter_2d();
        for row in 0..nrows {
            for column in 0..ncolumns {
                let pp = PointPlacement { row, column };
                let expected_val = Some(row * ncolumns + column);
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
        let mut board = Board::new(nrows, ncolumns);
        let nrows: usize = nrows.into();
        let ncolumns: usize = ncolumns.into();
        // [[0, 1, 2], [3, 4, 5], [6, 7, 8]]
        for row in 0..nrows {
            for column in 0..ncolumns {
                let val: usize = row * ncolumns + column;
                let pp = PointPlacement { row, column };
                board[pp] = Some(val);
            }
        }
        let mut result_iter = board.into_iter_2d();
        for row in 0..nrows {
            for column in 0..ncolumns {
                let pp = PointPlacement { row, column };
                let expected_val = Some(row * ncolumns + column);
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
        let mut board = Board::new(nrows, ncolumns);
        let mut board2 = Board::new(nrows, ncolumns);
        let nrows: usize = nrows.into();
        let ncolumns: usize = ncolumns.into();
        // [[0, 1, 2], [3, 4, 5], [6, 7, 8]]
        for row in 0..nrows {
            for column in 0..ncolumns {
                let val = row * ncolumns + column;
                let val2 = nrows * ncolumns - 1 - val;
                let pp = PointPlacement { row, column };
                board[pp] = Some(val);
                board2[pp] = Some(val2);
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
                    (pp, Some(expected_val), Some(expected_val2))
                );
            }
        }
        assert_eq!(result.next(), None);
    }
}
