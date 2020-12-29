pub mod karatsuba;

#[cfg(test)]
mod tests {
    use super::*;

    const SMALL_PRODUCTS: [(&str, &str, &str); 12] = [
        ("-10", "-10", "100"),
        ("-10", "10", "-100"),
        ("10", "10", "100"),
        ("10", "-10", "-100"),
        ("-25", "-25", "625"),
        ("-25", "25", "-625"),
        ("25", "25", "625"),
        ("25", "-25", "-625"),
        ("-100", "-100", "10000"),
        ("-100", "100", "-10000"),
        ("100", "100", "10000"),
        ("100", "-100", "-10000"),
    ];

    const LARGE_PRODUCTS: [(&str, &str, &str); 1] = [
        (
            "3141592653589793238462643383279502884197169399375105820974944592",
            "2718281828459045235360287471352662497757247093699959574966967627",
            "100"
        ),
    ];

    #[test]
    fn test_karatsuba_small_products() {
        for (x, y, expectation) in &SMALL_PRODUCTS {
            assert_eq!(karatsuba::multiply(*x, *y), *expectation);
        }
    }

    #[test]
    fn test_karatsuba_large_products() {
        for (x, y, expectation) in &LARGE_PRODUCTS {
            assert_eq!(karatsuba::multiply(*x, *y), *expectation);
        }
    }
}