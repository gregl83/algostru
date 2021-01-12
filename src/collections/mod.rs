pub mod multiplication;
pub mod ordering;
pub mod search;

fn midpoint<T>(x: &Vec<T>) -> usize {
    if x.is_empty() {
        return 0;
    }
    if x.len() % 2 == 0 {
        return x.len() / 2;
    }
    return (x.len() / 2) + 1;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_midpoint_index_even_length() {
        let x: Vec<isize> = vec![0, 1, 2, 3];

        let expectation_l = vec![0, 1];
        let expectation_r = vec![2, 3];

        let midpoint = midpoint(&x);

        assert_eq!(x[..midpoint].to_vec(), expectation_l);
        assert_eq!(x[midpoint..].to_vec(), expectation_r);
    }

    #[test]
    fn test_midpoint_index_odd_length() {
        let x: Vec<isize> = vec![0, 1, 2, 3, 4];

        let expectation_l = vec![0, 1, 2];
        let expectation_r = vec![3, 4];

        let midpoint = midpoint(&x);

        assert_eq!(x[..midpoint].to_vec(), expectation_l);
        assert_eq!(x[midpoint..].to_vec(), expectation_r);
    }
}