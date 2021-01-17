/// Insertion Sort
///
/// Input: mutable reference of vector x of n elements
/// Post-condition: sorted vector of n elements
///
/// ================================================================================================
///
/// ```ignore
/// loop over range from 2nd element to last in vector x
///     set insert_index to loop value
///     set current value using insert_index into vector x
///         while insert_index > 0 and previous value > current value
///             x[insert_index] = previous_value
///             subtract 1 from insert_index
///
///         x[insert_index] = current_value
/// ```
pub fn sort(x: &mut Vec<isize>) {
    let last_index = x.len();
    for i in 1..last_index {
        let mut insert_index = i;
        let current_value = x[insert_index];
        while insert_index > 0 && x[insert_index - 1] > current_value {
            x[insert_index] = x[insert_index - 1];
            insert_index -= 1;
        }
        x[insert_index] = current_value;
    }
}