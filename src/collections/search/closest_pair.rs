use crate::collections::midpoint;

enum Axis { X, Y }
type Point = (isize, isize);
type Plane = Vec<Point>;

fn get_point_axis(point: Point, axis: &Axis) -> isize {
    match axis {
        Axis::X => point.0,
        Axis::Y => point.1,
    }
}

fn merge_pairs(a: Plane, b: Plane, axis: &Axis) -> Plane {
    let mut merged: Plane = Vec::new();
    let mut a_index = 0;
    let mut b_index = 0;

    while a_index < a.len() && b_index < b.len() {
        if get_point_axis(a[a_index], &axis) <= get_point_axis(b[b_index], &axis) {
            merged.push(a[a_index]);
            a_index += 1;
        } else {
            merged.push(b[b_index]);
            b_index += 1;
        }
    }

    for element in a[a_index..].to_vec() {
        merged.push(element);
    }
    for element in b[b_index..].to_vec() {
        merged.push(element);
    }

    merged
}

fn sort_pairs(x: Plane, axis: &Axis) -> Plane {
    if x.len() <= 1 {
        return x;
    }

    let midpoint = midpoint(&x);
    let a = &x[..midpoint];
    let b = &x[midpoint..];

    let c = sort_pairs(a.to_vec(), &axis);
    let d = sort_pairs(b.to_vec(), &axis);

    merge_pairs(c, d, &axis)
}

// Closest Split Pair
//
// Input: Plane x of n Point elements
// Output: pair of Point elements from x that are closest
//
// =================================================================================================
//
// todo
fn closest_split_pair(px: Plane, py: Plane, delta: usize) -> (Point, Point) {
    let midpoint = midpoint(&px);
    let x_median = &px[midpoint];

    // Sy = q1 -> ql < delta

    (px[0], px[1])
}

// Closest Pair
//
// Input: Plane x of n Point elements
// Output: pair of Point elements from x that are closest
//
// =================================================================================================
//
// todo
fn closest_pair(px: Plane, py: Plane) -> (Point, Point) {
    if px.len() <= 3 {
        // fixme - base case - quicker to brute force
    }

    let midpoint = midpoint(&px);
    let lx = &px[..midpoint];
    let ly = &py[..midpoint];
    let rx = &px[midpoint..];
    let ry = &py[midpoint..];

    return (px[0], px[1])

    // todo - to_vec expensive; work in slices
    //let (l1, l2) = closest_pair(lx.to_vec(), ly.to_vec());
    //let (r1, r2) = closest_pair(rx.to_vec(), ry.to_vec());
    //let (s1, s2) = closest_split_pair(px, py);


    // fixme - return best of (l1, l2) or (r1, r2) or (s1, s2)
    //return (l1, l2)
}

// Find Closest Pair
//
// Input: Plane x of n Point elements
// Output: pair of Point elements from x that are closest
//
// =================================================================================================
//
// todo
pub fn find(x: Plane) -> (Point, Point) {
    let px: Plane = sort_pairs(x.clone(), &Axis::X); // todo - optimize O(n) clone w/borrow
    let py: Plane = sort_pairs(x, &Axis::Y);

    closest_pair(px, py)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_point_axis() {
        let point: Point = (0, 1);
        let axis = Axis::X;

        assert_eq!(get_point_axis(point, &Axis::X), point.0);
        assert_eq!(get_point_axis(point, &Axis::Y), point.1);
    }

    #[test]
    fn test_merge_pairs() {
        let a: Plane = vec![
            (0, 1),
            (1, 2)
        ];
        let b: Plane = vec![
            (2, 3),
            (3, 4),
            (4, 5)
        ];
        let axis = Axis::X;

        let expectation: Plane = vec![
            (0, 1),
            (1, 2),
            (2, 3),
            (3, 4),
            (4, 5)
        ];

        let result = merge_pairs(a, b, &axis);

        assert_eq!(result, expectation);
    }

    #[test]
    fn test_sort_pairs_x() {
        let pairs: Plane = vec![
            (4, 5),
            (3, 4),
            (2, 3),
            (1, 2),
            (0, 1)
        ];
        let axis = Axis::X;

        let expectation: Plane = vec![
            (0, 1),
            (1, 2),
            (2, 3),
            (3, 4),
            (4, 5)
        ];

        let result = sort_pairs(pairs, &axis);

        assert_eq!(result, expectation);
    }
}