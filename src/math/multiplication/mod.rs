pub mod karatsuba;

#[cfg(test)]
mod tests {
    use super::*;

    const SMALL_PRODUCTS: [(i64, i64, i64); 12] = [
        (-10, -10, 100),
        (-10, 10, -100),
        (10, 10, 100),
        (10, -10, -100),
        (-25, -25, 625),
        (-25, 25, -625),
        (25, 25, 625),
        (25, -25, -625),
        (-100, -100, 10000),
        (-100, 100, -10000),
        (100, 100, 10000),
        (100, -100, -10000),
    ];

    #[test]
    fn test_karatsuba_small_products() {
        for (x, y, expectation) in &SMALL_PRODUCTS {
            assert_eq!(karatsuba::multiply(*x, *y), *expectation);
        }
    }
}