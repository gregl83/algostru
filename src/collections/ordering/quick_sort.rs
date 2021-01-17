use rand::Rng;
use std::borrow::BorrowMut;

/// Generate random pivot index using start and end indices
fn rand_pivot(start: usize, end: usize) -> usize {
    rand::thread_rng().gen_range(start, end)
}

/// Partition vector of distinct integers (partially sort using endpoint indices)
fn partition(x: &mut [isize], left: usize, right: usize) -> usize {
    let pivot_value = x[left];
    let mut i = left + 1;
    for j in i..right {
        if x[j] < pivot_value {
            x.swap(j, i);
            i += 1;
        }

    }
    let pivot = i - 1;
    x.swap(left, pivot);
    pivot
}

/// Quick Sort implementation
fn quick_sort(x: &mut [isize], left: usize, right: usize) {
    if left >= right { return }

    let mut pivot = rand_pivot(left, right);
    x.swap(left, pivot);
    pivot = partition(x, left, right);

    quick_sort(x, left, pivot);
    quick_sort(x, pivot + 1, right);
}

/// Quick Sort
///
/// Input: vector x of n distinct integers, left and right endpoint indices
/// Post-condition: sorted vector of n elements
///
/// ================================================================================================
///
/// if length of x equals or is less than 1 then
///     base case: return
///
/// pivot index = random pivot index from x
/// swap x[left] for x[pivot index]
/// j = partition vector x (partial sort)
///
/// recursive sort:
///     x, left, j
///     x, j + 1, right
pub fn sort(x: &mut Vec<isize>) {
    let left = 0;
    let right = x.len();
    quick_sort(x.as_mut_slice(), left, right);
}