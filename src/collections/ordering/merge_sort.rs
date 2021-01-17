use crate::collections::vec_range_midpoint;

/// Merge Vectors
///
/// Input: two sorted vectors x and y
/// Output: merged vector containing the elements of x and y
///
/// =================================================================================================
///
/// result = new vector
/// x_index = 0
/// y_index = 0
///
/// while index_x < length of x and index_y < length of y
///     result.push smaller of x[x_index] and y[y_index]
///     increment index for smaller of x[x_index] and y[y_index] respectively
///
/// result.push for all elements remaining in x or y
///
/// return result
fn merge(x: Vec<isize>, y: Vec<isize>) -> Vec<isize> {
    let mut merged: Vec<isize> = Vec::new();
    let mut x_index = 0;
    let mut y_index = 0;

    while x_index < x.len() && y_index < y.len() {
        if x[x_index] <= y[y_index] {
            merged.push(x[x_index]);
            x_index += 1;
        } else {
            merged.push(y[y_index]);
            y_index += 1;
        }
    }

    for element in x[x_index..].to_vec() {
        merged.push(element);
    }
    for element in y[y_index..].to_vec() {
        merged.push(element);
    }

    merged
}

/// Merge Sort
///
/// Input: vector x of n elements
/// Output: sorted vector of n elements
///
/// ================================================================================================
///
/// if x length equals 0 or 1 then
///     base case: return vector x
///
/// recursive sort:
///   a = merge sort of first halve of x
///   b = merge sort of second halve of x
///
/// return result: vector x sorted
pub fn sort(x: Vec<isize>) -> Vec<isize> {
    if x.len() <= 1 {
        return x;
    }

    let midpoint = vec_range_midpoint(&x);
    let a = &x[..midpoint];
    let b = &x[midpoint..];

    let c = sort(a.to_vec());
    let d = sort(b.to_vec());

    merge(c, d)
}