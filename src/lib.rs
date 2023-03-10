use interfaces::*;
mod interfaces;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_() {
        let result_vec = vec![(PointPlacement { row: 0, column: 0 }, 0, 0)];
        let mut result = result_vec.into_iter();
        assert_eq!(
            result.next().unwrap(),
            (PointPlacement { row: 0, column: 0 }, 0, 0)
        );
    }
}
