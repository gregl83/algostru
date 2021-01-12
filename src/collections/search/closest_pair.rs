use std::f64;
use std::cmp::min;

use crate::collections::midpoint;
use std::borrow::Borrow;

enum Axis { X, Y }
type Point = (isize, isize);
type PointPair = (Point, Point);
type Plane = Vec<Point>;

fn get_point_axis(point: Point, axis: &Axis) -> isize {
    match axis {
        Axis::X => point.0,
        Axis::Y => point.1,
    }
}

fn euclidean_distance(a: Point, b: Point) -> f64 {
    let x_delta = (a.0 - b.0) as f64;
    let y_delta = (a.1 - b.1) as f64;
    (x_delta.powf(2.0) + y_delta.powf(2.0)).sqrt()
}

fn f64_min(vals: &[f64]) -> f64 {
    vals.iter().fold(f64::INFINITY, |a, &b| a.min(b))
}

fn iterate_closest_pair(x: &[PointPair]) -> &PointPair {
    let mut closest_pair: &(Point, Point) = &x[0];
    let mut best_euclidean_distance: f64 = f64::MAX;

    for (i, pair) in x.iter().enumerate() {
        let delta = euclidean_distance(pair.0, pair.1);

        if i == 0 {
            closest_pair = pair;
            best_euclidean_distance = delta;
            continue;
        }

        if f64_min(&[delta, best_euclidean_distance]) == delta {
            closest_pair = pair;
            best_euclidean_distance = delta;
            continue;
        }
    }

    closest_pair
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
fn closest_split_pair(px: Plane, py: Plane, delta: f64) -> Option<PointPair> {
    let midpoint = midpoint(&px);
    let pxl = &px[..midpoint];
    let x_median = pxl.last().unwrap().0 as f64;

    let sy: Vec<&Point> = py.iter().filter(|p| {
        let x = p.0 as f64;
        x >= (x_median - delta) && x <= (x_median + delta)
    }).collect();

    let mut closest_pair: Option<PointPair> = None;
    let mut best_euclidean_distance = delta;

    for i in 0..(sy.len() - 1) {
        let max_range  = min(7, sy.len() - i);
        for j in 1..max_range {
            let euclidean_distance = euclidean_distance(*sy[i], *sy[i + j]);
            if euclidean_distance < best_euclidean_distance {
                closest_pair = Some(
                    (sy[i].clone(), sy[i + j].clone())
                );
                best_euclidean_distance = euclidean_distance;
            }
        }
    }

    closest_pair
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
        return (px[0], px[1])
    }

    let midpoint = midpoint(&px);
    let lx = &px[..midpoint];
    let ly = &py[..midpoint];
    let rx = &px[midpoint..];
    let ry = &py[midpoint..];

    // todo - to_vec expensive; work in slices
    let (l1, l2) = closest_pair(lx.to_vec(), ly.to_vec());
    let (r1, r2) = closest_pair(rx.to_vec(), ry.to_vec());

    let delta = f64_min(
        &[
            euclidean_distance(l1, l2),
            euclidean_distance(r1, r2)
        ]
    );

    if let Some((s1, s2)) = closest_split_pair(px, py, delta) {
        return *iterate_closest_pair(&[
            (l1, l2),
            (r1, r2),
            (s1, s2),
        ]);
    }

    *iterate_closest_pair(&[
        (l1, l2),
        (r1, r2),
    ])
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
    fn test_f64_min() {
        let x = [2.0, 1.0, -10.0, 5.0, f64::NAN];
        assert_eq!(f64_min(&x), -10.0);
    }

    #[test]
    fn test_euclidean_distance() {
        let tests: [(Point, Point, f64); 3] = [
            ((2, 2), (2, 2), 0.0),
            ((-2, 2), (2, -2), 5.656854249492381),
            ((5, 6), (7, 8), 2.8284271247461903)
        ];

        for test in tests.to_vec() {
            assert_eq!(euclidean_distance(test.0, test.1), test.2);
        }
    }

    #[test]
    fn test_get_point_axis() {
        let point: Point = (0, 1);
        let axis = Axis::X;

        assert_eq!(get_point_axis(point, &Axis::X), point.0);
        assert_eq!(get_point_axis(point, &Axis::Y), point.1);
    }

    #[test]
    fn test_iterate_closest_pair() {
        let pairs: [(Point, Point); 3] = [
            ((0, 0), (1, 10)),
            ((1, 1), (2, 15)),
            ((2, 2), (3, 3)),
        ];
        let expectation = ((2, 2), (3, 3));

        let pair = iterate_closest_pair(&pairs);

        assert_eq!(pair, &expectation);
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