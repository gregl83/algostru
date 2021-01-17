use rand::Rng;
use std::borrow::BorrowMut;

fn rand_pivot(left: usize, right: usize) -> usize {
    rand::thread_rng().gen_range(left, right)
}

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

fn quick_sort(x: &mut [isize], left: usize, right: usize) {
    if left >= right { return }

    let mut pivot = rand_pivot(left, right);
    x.swap(left, pivot);
    pivot = partition(x, left, right);

    quick_sort(x, left, pivot);
    quick_sort(x, pivot + 1, right);
}

// Quick Sort
//
// Input: vector x of n elements
// Output: sorted vector of n elements
//
// =================================================================================================
//
// todo
pub fn sort(x: &mut Vec<isize>) {
    let left = 0;
    let right = x.len();
    quick_sort(x.as_mut_slice(), left, right);
}