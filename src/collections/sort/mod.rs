pub mod merge_sort;
pub mod quick_sort;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_sort() {
        let x: Vec<isize> = vec![
            10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0, -1, -2, -3, -4, -5, -6, -7, -8, -9, -10
        ];
        let expectation: Vec<isize> = vec![
            -10, -9, -8, -7, -6, -5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10
        ];

        let sorted = merge_sort::sort(x);

        assert_eq!(sorted, expectation);
    }

    #[test]
    fn test_quick_sort() {
        let x: Vec<isize> = vec![
            10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0, -1, -2, -3, -4, -5, -6, -7, -8, -9, -10
        ];
        let expectation: Vec<isize> = vec![
            -10, -9, -8, -7, -6, -5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10
        ];

        let sorted = quick_sort::sort(x);

        assert_eq!(sorted, expectation);
    }
}