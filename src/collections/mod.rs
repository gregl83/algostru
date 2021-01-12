pub mod multiplication;
pub mod ordering;
pub mod search;

// geometric types
enum Axis { X, Y }
type Point = (isize, isize);
type PointPair = (Point, Point);
type Plane = Vec<Point>;

fn f64_min(vals: &[f64]) -> f64 {
    vals.iter().fold(f64::INFINITY, |a, &b| a.min(b))
}

fn get_point_value_by_axis(p: Point, axis: &Axis) -> isize {
    match axis {
        Axis::X => p.0,
        Axis::Y => p.1,
    }
}

fn vec_range_midpoint<T>(x: &Vec<T>) -> usize {
    if x.is_empty() {
        return 0;
    }
    if x.len() % 2 == 0 {
        return x.len() / 2;
    }
    return (x.len() / 2) + 1;
}

fn euclidean_distance(a: Point, b: Point) -> f64 {
    let x_delta = (a.0 - b.0) as f64;
    let y_delta = (a.1 - b.1) as f64;
    (x_delta.powf(2.0) + y_delta.powf(2.0)).sqrt()
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
    fn test_get_point_value_by_axis() {
        let point: Point = (0, 1);
        assert_eq!(get_point_value_by_axis(point, &Axis::X), point.0);
        assert_eq!(get_point_value_by_axis(point, &Axis::Y), point.1);
    }

    #[test]
    fn test_even_length_vec_range_midpoint() {
        let x: Vec<isize> = vec![0, 1, 2, 3];

        let expectation_l = vec![0, 1];
        let expectation_r = vec![2, 3];

        let midpoint = vec_range_midpoint(&x);

        assert_eq!(x[..midpoint].to_vec(), expectation_l);
        assert_eq!(x[midpoint..].to_vec(), expectation_r);
    }

    #[test]
    fn test_odd_length_vec_range_midpoint() {
        let x: Vec<isize> = vec![0, 1, 2, 3, 4];

        let expectation_l = vec![0, 1, 2];
        let expectation_r = vec![3, 4];

        let midpoint = vec_range_midpoint(&x);

        assert_eq!(x[..midpoint].to_vec(), expectation_l);
        assert_eq!(x[midpoint..].to_vec(), expectation_r);
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
}