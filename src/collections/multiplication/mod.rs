pub mod strassen;

#[cfg(test)]
mod tests {
    use super::*;

    use nalgebra::DMatrix;

    #[test]
    fn test_strassen_products() {
        let x = DMatrix::from_row_slice(4, 4, &[
            10, 9, 4, 3,
            8, 3, 4, 1,
            93, 1, 9, 3,
            2, 2, 7, 6
        ]);

        let y = DMatrix::from_row_slice(4, 4, &[
            4, 5, 3, 5,
            4, 1, 2, 1,
            9, 8, 3, 5,
            6, 3, 7, 9
        ]);

        let expectation = DMatrix::from_row_slice(4, 4, &[
            130, 100, 81, 106,
            86, 78, 49, 72,
            475, 547, 329, 538,
            115, 86, 73, 101
        ]);

        let product = strassen::multiply(x, y);

        assert_eq!(product, expectation);
    }
}