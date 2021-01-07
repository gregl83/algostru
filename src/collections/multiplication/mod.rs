pub mod strassen;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strassen_products() {
        let x: Vec<Vec<isize>> = vec![
            vec![10, 9, 4, 3],
            vec![8, 3, 4, 1],
            vec![93, 1, 9, 3],
            vec![2, 2, 7, 6]
        ];

        let y: Vec<Vec<isize>> = vec![
            vec![4, 5, 3, 5],
            vec![4, 1, 2, 1],
            vec![9, 8, 3, 5],
            vec![6, 3, 7, 9]
        ];

        let expectation: Vec<Vec<isize>> = vec![
            vec![130, 100, 81, 106],
            vec![86, 78, 49, 72],
            vec![475, 547, 329, 538],
            vec![115, 86, 73, 101]
        ];

        let product = strassen::multiply(x, y);

        assert_eq!(product, expectation);
    }
}