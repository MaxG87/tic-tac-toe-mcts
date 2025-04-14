use crate::interfaces::{BoardSizeT, BoardStateEntry, PointPlacement};
use std::ops::IndexMut;
use std::{
    iter::{zip, Iterator},
    ops::Index,
};

#[derive(Debug, Clone, PartialEq, Eq)]
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

    pub fn has_placement_at(&self, pp: PointPlacement) -> bool {
        let index = self.to_index(pp);
        self.board[index].is_some()
    }

    pub fn iter_2d(
        &self,
    ) -> impl Iterator<Item = (BoardSizeT, BoardSizeT, &BoardStateEntry)> {
        self.board.iter().enumerate().map(|(index, val)| {
            let row = index / usize::from(self.ncolumns);
            let column = index % usize::from(self.ncolumns);
            (row, column, val)
        })
    }

    pub fn into_iter_2d(
        self,
    ) -> impl Iterator<Item = (BoardSizeT, BoardSizeT, BoardStateEntry)> {
        self.board.into_iter().enumerate().map(move |(index, val)| {
            let row = index / usize::from(self.ncolumns);
            let column = index % usize::from(self.ncolumns);
            (row, column, val)
        })
    }

    pub fn joint_iter_2d(
        self,
        board2: Board,
    ) -> impl Iterator<Item = (BoardSizeT, BoardSizeT, BoardStateEntry, BoardStateEntry)>
    {
        zip(self.into_iter_2d(), board2.into_iter_2d())
            .map(|(lhs, rhs)| (lhs.0, lhs.1, lhs.2, rhs.2))
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

pub fn iter_2d_array<T, const N: BoardSizeT>(
    array: &[[T; N]; N],
) -> impl Iterator<Item = (BoardSizeT, BoardSizeT, &T)> {
    array
        .iter()
        .enumerate()
        .flat_map(|(r, row)| row.iter().enumerate().map(move |(c, val)| (r, c, val)))
}

pub fn iter_mut_2d_array<T, const N: BoardSizeT>(
    array: &mut [[T; N]; N],
) -> impl Iterator<Item = (BoardSizeT, BoardSizeT, &mut T)> {
    array.iter_mut().enumerate().flat_map(|(r, row)| {
        row.iter_mut().enumerate().map(move |(c, val)| (r, c, val))
    })
}

pub fn into_iter_2d_array<T: Clone, const N: BoardSizeT>(
    array: &[[T; N]; N],
) -> impl Iterator<Item = (BoardSizeT, BoardSizeT, T)> + '_ {
    iter_2d_array(array).map(|(r, c, v)| (r, c, v.clone()))
}

pub fn joint_iter_2d_arrays<
    ValueT1,
    ValueT2,
    IterT1: Iterator<Item = (BoardSizeT, BoardSizeT, ValueT1)>,
    IterT2: Iterator<Item = (BoardSizeT, BoardSizeT, ValueT2)>,
>(
    array_iter1: IterT1,
    array_iter2: IterT2,
) -> impl Iterator<Item = (BoardSizeT, BoardSizeT, ValueT1, ValueT2)> {
    zip(array_iter1, array_iter2).map(|(lhs, rhs)| (lhs.0, lhs.1, lhs.2, rhs.2))
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
                let expected_val = Some(row * ncolumns + column);
                let result = result_iter.next().unwrap();
                assert_eq!(result, (row, column, &expected_val));
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
                let expected_val = Some(row * ncolumns + column);
                let result = result_iter.next().unwrap();
                assert_eq!(result, (row, column, expected_val));
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
                let expected_val = row * ncolumns + column;
                let expected_val2 = nrows * ncolumns - 1 - expected_val;

                assert_eq!(
                    result.next().unwrap(),
                    (row, column, Some(expected_val), Some(expected_val2))
                );
            }
        }
        assert_eq!(result.next(), None);
    }
}
