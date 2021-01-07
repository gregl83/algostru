type Matrix = Vec<Vec<isize>>;

// Quarter Matrix
//
// Input: n-vector vector x
// Output: tuple of vector x quartered
// Assumption: x is square matrix
//
// =================================================================================================
//
// todo
fn quarter_matrix(x: Matrix) -> (Matrix, Matrix, Matrix, Matrix) {
    
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
    // fixme - implement strassen
    x
}

pub fn multiply(x: Matrix, y: Matrix) -> Matrix {
    strassen(x, y)
}