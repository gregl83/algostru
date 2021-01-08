use std::ops::{Div, Mul};

use nalgebra::{DMatrix, DMatrixSlice, Matrix};
use num_integer::Integer;

type M = DMatrix<isize>;
type MSlice<'a> = DMatrixSlice<'a, isize>;

fn to_slice(m: &M) -> MSlice {
    m.slice((0, 0), (m.nrows(), m.ncols()))
}

// Quarter Matrix
//
// Input: reference to matrix slice x
// Output: tuple of matrix slice x quartered
// Assumption: x is square matrix
//
// =================================================================================================
//
// x_rows, x_cols = x matrix shape
// midpoint_row, midpoint_col = floored halves of x_rows and x_cols
//
// return result: tuple of matrix x quarters
fn quarter_matrix<'a>(x: &'a MSlice<'a>) -> (MSlice<'a>, MSlice<'a>, MSlice<'a>, MSlice<'a>) {
    let divisor: usize = 2;

    let midpoint_row = x.nrows().div_floor(&divisor);
    let midpoint_cols = x.ncols().div_floor(&divisor);

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

// Combine Quarters
//
// Input: matrix quarters q1..q4
// Output: matrix of quarters combined
// Assumption: quarters are square matrices
//
// =================================================================================================
//
// quarters = (q1..q4)
// n_rows = q1_rows + q3_rows
// n_cols = q1_cols + q2_cols
//
// data = []
// loop quarters
//     loop quarter values
//         insert value into data
//
// return result: matrix of size n_rows x n_cols with data
fn combine_quarters(q1: M, q2: M, q3: M, q4: M, ) -> M {
    let quarters = [&q1, &q4, &q2, &q3];
    let n_rows = &q1.nrows() * &q3.nrows();
    let n_cols = &q1.ncols() * &q2.ncols();

    let mut data = vec![];

    for q in quarters.iter() {
        for val in q.iter() {
            data.push(*val);
        }
    }

    DMatrix::from_vec(n_rows, n_cols, data)
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
fn strassen(x: MSlice, y: MSlice) -> M {
    if x.nrows() == 1 && x.ncols() == 1 {
        return x * y;
    }

    let (a, b, c, d) = quarter_matrix(&x);
    let (e, f, g, h) = quarter_matrix(&y);

    let p1_y: M = f - h;
    let p1 = strassen(a, to_slice(&p1_y));

    let p2_x: M = a - b;
    let p2 = strassen(to_slice(&p2_x), h);

    let p3_x: M = c + d;
    let p3 = strassen(to_slice(&p3_x), e);

    let p4_y: M = g - e;
    let p4 = strassen(d, to_slice(&p4_y));

    let p5_x: M = a + d;
    let p5_y: M = e + h;
    let p5 = strassen(to_slice(&p5_x), to_slice(&p5_y));

    let p6_x: M = b - d;
    let p6_y: M = g + h;
    let p6 = strassen(to_slice(&p6_x), to_slice(&p6_y));

    let p7_x: M = a - c;
    let p7_y: M = e + f;
    let p7 = strassen(to_slice(&p7_x), to_slice(&p7_y));

    let q1: M = &p5 + &p4 - &p2 + &p6;
    let q2: M = &p1 + &p2;
    let q3: M = &p3 + &p4;
    let q4: M = &p1 + &p5 - &p3 - &p7;

    combine_quarters(q1, q2, q3, q4)
}

pub fn multiply(x: M, y: M) -> M {
    strassen(to_slice(&x), to_slice(&y))
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

        let x_slice = to_slice(&x);

        let (q1, q2, q3, q4) = quarter_matrix(&x_slice);

        assert_eq!(q1, expectation[0]);
        assert_eq!(q2, expectation[1]);
        assert_eq!(q3, expectation[2]);
        assert_eq!(q4, expectation[3]);
    }

    #[test]
    fn test_combine_quarters() {
        let q1 = DMatrix::from_row_slice(2, 2, &[
            10, 9,
            8, 3
        ]);
        let q2 = DMatrix::from_row_slice(2, 2, &[
            4, 3,
            4, 1
        ]);
        let q3 = DMatrix::from_row_slice(2, 2, &[
            93, 1,
            2, 2
        ]);
        let q4 = DMatrix::from_row_slice(2, 2, &[
            9, 3,
            7, 6
        ]);

        let expectation = DMatrix::from_row_slice(4, 4, &[
            10, 9, 4, 3,
            8, 3, 4, 1,
            93, 1, 9, 3,
            2, 2, 7, 6
        ]);

        let result = combine_quarters(q1, q2, q3, q4);

        assert_eq!(result, expectation);
    }
}