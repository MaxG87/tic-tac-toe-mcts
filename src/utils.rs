use crate::interfaces::BoardSizeT;
use std::iter::*;

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

    #[test]
    fn test_iter_2d_array() {
        const N: BoardSizeT = 3;
        let array = [[0, 1, 2], [3, 4, 5], [6, 7, 8]];
        let mut result = iter_2d_array(&array);
        for row in 0..3 {
            for column in 0..3 {
                let expected_val = row * N + column;

                assert_eq!(result.next().unwrap(), (row, column, &expected_val));
            }
        }
        assert_eq!(result.next(), None);
    }

    #[test]
    fn test_into_iter_2d_array() {
        const N: BoardSizeT = 3;
        let array = [[0, 1, 2], [3, 4, 5], [6, 7, 8]];
        let mut result = into_iter_2d_array(&array);
        for row in 0..3 {
            for column in 0..3 {
                let expected_val = row * N + column;

                assert_eq!(result.next().unwrap(), (row, column, expected_val));
            }
        }
        assert_eq!(result.next(), None);
    }

    #[test]
    fn test_joint_iter_2d_arrays() {
        const N: BoardSizeT = 3;
        let array = [[0, 1, 2], [3, 4, 5], [6, 7, 8]];
        let array2 = [[8, 7, 6], [5, 4, 3], [2, 1, 0]];
        let mut result = joint_iter_2d_arrays(
            into_iter_2d_array(&array),
            into_iter_2d_array(&array2),
        );
        for row in 0..3 {
            for column in 0..3 {
                let expected_val = row * N + column;

                assert_eq!(
                    result.next().unwrap(),
                    (row, column, expected_val, 8 - expected_val)
                );
            }
        }
        assert_eq!(result.next(), None);
    }
}
