use crate::interfaces::BoardSizeT;
use std::iter::{zip, Iterator};

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
