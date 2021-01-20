use std::cmp;

/// Insertion Sort with left/right index boundaries
fn insertion_sort(x: &mut Vec<isize>, left: usize, right: Option<usize>) {
    let right = match right {
        Some(right) => right,
        _ => x.len()
    };

    for i in (left + 1)..right {
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
fn merge(x: &[isize], y: &[isize]) -> Vec<isize> {
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
        insertion_sort(x, i, Some(cmp::min((i + min_run - 1), n - 1)))
    }

    let mut size = min_run;
    while size < n {
        for start in (0..n).step_by(size * 2) {
            let midpoint = start + size -1;
            let end = cmp::min((start + size * 2), n - 1);

            let merged = merge(&x[start..(midpoint + 1)], &x[(midpoint + 1)..(end + 1)]);

            x[start..(start + merged.len())] = *merged;
        }
        size *= 2;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insertion_sort_with_boundaries() {
        let mut x: Vec<isize> = vec![
            10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0, -1, -2, -3, -4, -5, -6, -7, -8, -9, -10
        ];
        let expectation: Vec<isize> = vec![
            -10, -9, -8, -7, -6, -5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10
        ];

        insertion_sort(&mut x, 0, None);

        assert_eq!(x, expectation);
    }
}