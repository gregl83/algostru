use std::ops::Div;
use num_integer::Integer;

type Matrix = Vec<Vec<isize>>;

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
fn quarter_matrix(x: Matrix) -> (Matrix, Matrix, Matrix, Matrix) {
    let two: usize = 2;

    let x_rows = x.len();
    let x_cols = x[0].len();

    let midpoint_row = x_rows.div_floor(&two);
    let midpoint_cols = x_cols.div_floor(&two);

    let mut q1 = vec![];
    for row in x[..midpoint_row].iter() {
        q1.push(row[..midpoint_cols].to_vec());
    }

    let mut q2 = vec![];
    for row in x[..midpoint_row].iter() {
        q2.push(row[midpoint_cols..].to_vec());
    }

    let mut q3 = vec![];
    for row in x[midpoint_row..].iter() {
        q3.push(row[..midpoint_cols].to_vec());
    }

    let mut q4 = vec![];
    for row in x[midpoint_row..].iter() {
        q4.push(row[midpoint_cols..].to_vec());
    }

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
fn strassen(x: Matrix, y: Matrix) -> Matrix {
    x
}

pub fn multiply(x: Matrix, y: Matrix) -> Matrix {
    strassen(x, y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quarter_matrix() {
        let x: Vec<Vec<isize>> = vec![
            vec![10, 9, 4, 3],
            vec![8, 3, 4, 1],
            vec![93, 1, 9, 3],
            vec![2, 2, 7, 6]
        ];

        let expectation: Vec<Vec<Vec<isize>>> = vec![
            vec![
                vec![10, 9],
                vec![8, 3]
            ],
            vec![
                vec![4, 3],
                vec![4, 1]
            ],
            vec![
                vec![93, 1],
                vec![2, 2]
            ],
            vec![
                vec![9, 3],
                vec![7, 6]
            ]
        ];

        let (q1, q2, q3, q4) = quarter_matrix(x);

        assert_eq!(q1, expectation[0]);
        assert_eq!(q2, expectation[1]);
        assert_eq!(q3, expectation[2]);
        assert_eq!(q4, expectation[3]);
    }
}