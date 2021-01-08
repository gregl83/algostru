type Point = (isize, isize);
type Plane = Vec<Point>;

// Closest Pair
//
// Input: Plane x of n Point elements
// Output: pair of Point elements from x that are closest
//
// =================================================================================================
//
// todo
fn closest_pair(x: Plane) -> (Point, Point) {
    (x[0].0, x[0].1)
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
    closest_pair(x)
}