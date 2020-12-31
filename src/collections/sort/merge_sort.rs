// Merge Vectors
//
// Input: two sorted vectors x and y
// Output: merged vector containing the elements of x and y
//
// =================================================================================================
//
// result = new vector
// x_index = 0
// y_index = 0
//
// while index_x < length of x and index_y < length of y
//     result.push smaller of x[x_index] and y[y_index]
//     increment index for smaller of x[x_index] and y[y_index] respectively
//
// result.push for all elements remaining in x or y
//
// return result
fn merge(x: Vec<isize>, y: Vec<isize>) -> Vec<isize> {

}

// Merge Sort
//
// Input: vector x of n elements
// Output: sorted vector of n elements
//
// =================================================================================================
//
// if x length equals 0 or 1 then
//     base case: return vector
//
//     recursive sort:
//     a = merge sort of first halve of x
//     b = merge sort of second halve of x
//
//     return result: x sorted
pub fn sort(x: Vec<isize>) -> Vec<isize> {
    if x.len() <= 1 {
        return x;
    }

    let midpoint = x.len() / 2;
    let a = x[midpoint..];
    let b = x[..midpoint];

    let c = merge_sort(a.to_vec());
    let d = merge_sort(b.to_vec());

    merge(c, d)
}