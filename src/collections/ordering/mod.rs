pub mod bubble_sort;
pub mod merge_sort;
pub mod merge_sort_inversion_count;
pub mod quick_sort;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bubble_sort() {
        let mut x: Vec<isize> = vec![
            10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0, -1, -2, -3, -4, -5, -6, -7, -8, -9, -10
        ];
        let expectation: Vec<isize> = vec![
            -10, -9, -8, -7, -6, -5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10
        ];

        bubble_sort::sort(&mut x);

        assert_eq!(x, expectation);
    }

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
    fn test_merge_sort_inversion_count() {
        let x: Vec<isize> = vec![
            10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0, -1, -2, -3, -4, -5, -6, -7, -8, -9, -10
        ];
        let expectation: (Vec<isize>, usize) = (
            vec![
                -10, -9, -8, -7, -6, -5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10
            ],
            210
        );

        let sorted_counted = merge_sort_inversion_count::sort(x);

        assert_eq!(sorted_counted, expectation);
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