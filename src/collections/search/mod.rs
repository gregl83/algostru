pub mod closest_pair;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_closest_pair() {
        let plane = vec![
            (0, 2),
            (4, 6),
            (6, 8),
            (10, 12),
            (11, 13)
        ];

        let pair = closest_pair::find(plane.clone());

        assert_eq!(pair, (plane[3], plane[4]));
    }

    #[test]
    fn test_closest_split_pair() {
        let plane = vec![
            (0, 2),
            (4, 6),
            (1, 3),
            (10, 12)
        ];

        let pair = closest_pair::find(plane.clone());

        assert_eq!(pair, (plane[0], plane[2]));
    }
}