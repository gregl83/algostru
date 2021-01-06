// Bubble Sort
//
// Input: mutable reference of vector x of n elements
// Output: sorted vector of n elements
//
// =================================================================================================
//
// loop n elements in x setting index to i
//     loop n elements in x starting from index i setting index to j
//         if index j element is greater than index j + 1 element perform swap
//
pub fn sort(x: &mut Vec<isize>) {
    let last_index = x.len() - 1;
    for i in 0..last_index {
        for j in 0..last_index - i {
            let next = j + 1;
            if x[j] > x[next] {
                x.swap(j, next);
            }
        }
    }
}