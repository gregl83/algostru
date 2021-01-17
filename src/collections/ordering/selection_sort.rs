/// Selection Sort
///
/// Input: mutable reference of vector x of n elements
/// Post-condition: sorted vector of n elements
///
/// ================================================================================================
///
/// ```ignore
/// loop n elements in x setting index to i
///     set minimum_index = i
///     loop i + 1 elements in x setting index to j
///         if x[j] < x[minimum_index] then
///             minimum_index = j
///     swap minimum index in x with i in x
/// ```
pub fn sort(x: &mut Vec<isize>) {
    let last_index = x.len();
    for i in 0..last_index {
        let mut minimum_index = i;
        for j in (i + 1)..last_index {
            if x[j] < x[minimum_index] {
                minimum_index = j;
            }
        }
        x.swap(minimum_index, i);
    }
}