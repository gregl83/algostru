// Inversion Count
//
// Input: vector x of n elements
// Output: sorted vector of n elements and number of inversions found in input vector x
//
// =================================================================================================
//
// if x length equals 0 or 1 then
//     base case: return vector x
//
// recursive sort:
//   a = sort merge and count inversions in first halve of x
//   b = sort merge and count inversions in second halve of x
//   c = sort merge and count of split inversions in vectors a and b
//
// return result: sort merge c and number of inversions in a + b + c
pub fn count(x: Vec<isize>) -> (Vec<isize>, isize) {
    x
}