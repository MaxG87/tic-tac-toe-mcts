use std::iter::*;

pub fn iter_2d_array<T, const N: usize>(
    array: &[[T; N]; N],
) -> impl Iterator<Item = (usize, usize, &T)> {
    array
        .iter()
        .enumerate()
        .flat_map(|(r, row)| row.iter().enumerate().map(move |(c, val)| (r, c, val)))
}

pub fn iter_mut_2d_array<T, const N: usize>(
    array: &mut [[T; N]; N],
) -> impl Iterator<Item = (usize, usize, &mut T)> {
    array
        .iter_mut()
        .enumerate()
        .flat_map(|(r, row)| row.iter_mut().enumerate().map(move |(c, val)| (r, c, val)))
}

pub fn into_iter_2d_array<T: Clone, const N: usize>(
    array: &[[T; N]; N],
) -> impl Iterator<Item = (usize, usize, T)> + '_ {
    iter_2d_array(array).map(|(r, c, v)| (r, c, v.clone()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iter_2d_array() {
        const N: usize = 3;
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
        const N: usize = 3;
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
}
