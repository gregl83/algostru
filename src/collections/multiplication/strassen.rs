use std::ops::{Div, Mul};

use nalgebra::{
    DMatrix,
    DMatrixSlice
};
use num_integer::Integer;

// Quarter Matrix
//
// Input: n-vector vector x
// Output: tuple of vector x quartered
// Assumption: x is square matrix
//
// =================================================================================================
//
// x_rows, x_cols = x matrix shape
// midpoint_row, midpoint_col = floored halves of x_rows and x_cols
//
// return result: quartered matrix x wrapped in tuple
fn quarter_matrix(x: &DMatrix<isize>) -> (
    DMatrixSlice<isize>,
    DMatrixSlice<isize>,
    DMatrixSlice<isize>,
    DMatrixSlice<isize>
) {
    let two: usize = 2;

    let midpoint_row = x.nrows().div_floor(&two);
    let midpoint_cols = x.ncols().div_floor(&two);

    let q1 = x.slice(
        (0, 0),
        (midpoint_row, midpoint_cols)
    );
    let q2 = x.slice(
        (0, midpoint_cols),
        (midpoint_row, x.ncols() - midpoint_cols)
    );
    let q3 = x.slice(
        (midpoint_row, 0),
        (x.nrows() - midpoint_row, midpoint_cols)
    );
    let q4 = x.slice(
        (midpoint_row, midpoint_cols),
        (x.nrows() - midpoint_row, x.ncols() - midpoint_cols)
    );

    (q1, q2, q3, q4)
}

// Strassen Matrix Multiplication
//
// Input: n-vector vectors x and y
// Output: n-vector vector product of x and y
// Assumption: x and y are equal squares
//
// =================================================================================================
//
// todo
fn strassen(x: DMatrix<isize>, y: DMatrix<isize>) -> DMatrix<isize> {
    if x.nrows() == 1 && x.ncols() == 1 {
        return x * y;
    }

    x
}

// def strassen(x, y):
// """
//     Computes matrix product by divide and conquer approach, recursively.
//     Input: nxn matrices x and y
//     Output: nxn matrix, product of x and y
//     """
//
// # Base case when size of matrices is 1x1
// if len(x) == 1:
// return x * y
//
// # Splitting the matrices into quadrants. This will be done recursively
// # untill the base case is reached.
// a, b, c, d = split(x)
// e, f, g, h = split(y)
//
// # Computing the 7 products, recursively (p1, p2...p7)
// p1 = strassen(a, f - h)
// p2 = strassen(a + b, h)
// p3 = strassen(c + d, e)
// p4 = strassen(d, g - e)
// p5 = strassen(a + d, e + h)
// p6 = strassen(b - d, g + h)
// p7 = strassen(a - c, e + f)
//
// # Computing the values of the 4 quadrants of the final matrix c
// c11 = p5 + p4 - p2 + p6
// c12 = p1 + p2
// c21 = p3 + p4
// c22 = p1 + p5 - p3 - p7
//
// # Combining the 4 quadrants into a single matrix by stacking horizontally and vertically.
// c = np.vstack((np.hstack((c11, c12)), np.hstack((c21, c22))))
//
// return c

pub fn multiply(x: DMatrix<isize>, y: DMatrix<isize>) -> DMatrix<isize> {
    strassen(x, y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quarter_matrix() {
        let x = DMatrix::from_row_slice(4, 4, &[
            10, 9, 4, 3,
            8, 3, 4, 1,
            93, 1, 9, 3,
            2, 2, 7, 6
        ]);

        let expectation: Vec<DMatrix<isize>> = vec![
            DMatrix::from_row_slice(2, 2, &[
                10, 9,
                8, 3
            ]),
            DMatrix::from_row_slice(2, 2, &[
                4, 3,
                4, 1
            ]),
            DMatrix::from_row_slice(2, 2, &[
                93, 1,
                2, 2
            ]),
            DMatrix::from_row_slice(2, 2, &[
                9, 3,
                7, 6
            ])
        ];

        let (q1, q2, q3, q4) = quarter_matrix(&x);

        assert_eq!(q1, expectation[0]);
        assert_eq!(q2, expectation[1]);
        assert_eq!(q3, expectation[2]);
        assert_eq!(q4, expectation[3]);
    }
}