use crate::collections::vec_range_midpoint;

/// Merge Vectors and Count Inversions
///
/// Input: two sorted vectors x and y
/// Output: merged vector containing the elements of x and y with inversion count
///
/// ================================================================================================
///
/// result = new vector
/// x_index = 0
/// y_index = 0
/// split_inversion_count = 0
///
/// while index_x < length of x and index_y < length of y
///     result.push smaller of x[x_index] and y[y_index]
///     increment index for smaller of x[x_index] and y[y_index] respectively
///     increment split inversion count when x[x_index] > y[y_index] by length of x - index_x + 1
///
/// result.push for all elements remaining in x or y
///
/// return result and split inversion count
fn merge_count(x: Vec<isize>, y: Vec<isize>) -> (Vec<isize>, usize) {
    let mut merged: Vec<isize> = Vec::new();
    let mut x_index = 0;
    let mut y_index = 0;
    let mut split_inversions: usize = 0;

    while x_index < x.len() && y_index < y.len() {
        if x[x_index] <= y[y_index] {
            merged.push(x[x_index]);
            x_index += 1;
        } else {
            merged.push(y[y_index]);
            y_index += 1;
            split_inversions += x.len() - x_index;
        }
    }

    for element in x[x_index..].to_vec() {
        merged.push(element);
    }
    for element in y[y_index..].to_vec() {
        merged.push(element);
    }

    (merged, split_inversions)
}

/// Merge Sort Inversion Count
///
/// Input: vector x of n elements
/// Output: sorted vector of n elements and number of inversions found in input vector x
///
/// ================================================================================================
///
/// if x length equals 0 or 1 then
///     base case: return vector x
///
/// recursive sort:
///   a = sort merge and count inversions in first halve of x
///   b = sort merge and count inversions in second halve of x
///   c = sort merge and count of split inversions in vectors a and b
///
/// return result: sort merge c and number of inversions in a + b + c
pub fn sort(x: Vec<isize>) -> (Vec<isize>, usize) {
    if x.len() <= 1 {
        return (x, 0);
    }

    let midpoint = vec_range_midpoint(&x);
    let a = &x[..midpoint];
    let b = &x[midpoint..];

    let (c, c_inversions) = sort(a.to_vec());
    let (d, d_inversions) = sort(b.to_vec());

    let (e, e_inversions) = merge_count(c, d);

    (e, c_inversions + d_inversions + e_inversions)
}