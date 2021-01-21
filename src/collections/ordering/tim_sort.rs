use std::cmp;

/// Insertion Sort with left/right index boundaries
fn insertion_sort(x: &mut Vec<isize>, left: usize, right: Option<usize>) {
    let right = match right {
        Some(right) => right,
        _ => x.len()
    };

    for i in left..right {
        let mut insert_index = i;
        let current_value = x[insert_index];
        while insert_index > left && x[insert_index - 1] > current_value {
            x[insert_index] = x[insert_index - 1];
            insert_index -= 1;
        }
        x[insert_index] = current_value;
    }
}

/// Merge Sort with left/right sides (x/y)
fn merge(x: &mut Vec<isize>, start: usize, midpoint: usize, end: usize) {
    let mut sorted: Vec<isize> = Vec::new();

    let mut start = start;
    let mut midpoint = midpoint;

    if midpoint >= end {
        midpoint = start;
        start = 0;
    }

    let left = &x[start..midpoint];
    let right = &x[midpoint..end];

    let mut left_index = 0;
    let mut right_index = 0;

    while left_index < left.len()  && right_index < right.len() {
        if left[left_index] <= right[right_index] {
            sorted.push(left[left_index]);
            left_index += 1;
        } else {
            sorted.push(right[right_index]);
            right_index += 1;
        }
    }

    while left_index < left.len() {
        sorted.push(left[left_index]);
        left_index += 1;
    }
    while right_index < right.len() {
        sorted.push(right[right_index]);
        right_index += 1;
    }

    let mut index_start = start;
    for element in sorted.iter() {
        x[index_start] = *element;
        index_start += 1;
    }
}

/// Time Sort
///
/// Input: mutable reference of vector x of n elements
/// Post-condition: sorted vector of n elements
///
/// ================================================================================================
///
/// fixme
pub fn sort(x: &mut Vec<isize>) {
    let min_run = 32;
    let n = x.len();

    for i in (0..n).step_by(min_run) {
        insertion_sort(x, i, Some(cmp::min((i + min_run), n)));
    }

    let mut size = min_run;
    while size < n {
        for start in (0..n).step_by(size * 2) {
            let midpoint = start + size;
            let end = cmp::min((start + size * 2), n);

            merge(x, start, midpoint, end);
        }
        size *= 2;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insertion_sort_with_total_boundaries() {
        let mut x: Vec<isize> = vec![
            10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0, -1, -2, -3, -4, -5, -6, -7, -8, -9, -10
        ];
        let expectation: Vec<isize> = vec![
            -10, -9, -8, -7, -6, -5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10
        ];

        insertion_sort(&mut x, 0, None);

        assert_eq!(x, expectation);
    }

    #[test]
    fn test_insertion_sort_with_subset_boundaries() {
        let mut x: Vec<isize> = vec![
            10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0, -1, -2, -3, -4, -5, -6, -7, -8, -9, -10
        ];
        let expectation: Vec<isize> = vec![
            10, 9, 8, 7, 6, -5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5, -6, -7, -8, -9, -10
        ];

        insertion_sort(&mut x, 5, Some(16));

        assert_eq!(x, expectation);
    }
}