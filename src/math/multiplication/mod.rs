pub mod karatsuba;

#[cfg(test)]
mod tests {
    use super::*;

    use num_bigint::{BigInt, ToBigInt};

    const SMALL_PRODUCTS: [(isize, isize, isize); 12] = [
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

    const BIG_PRODUCTS: [(&[u8], &[u8], &[u8]); 1] = [
        (
            b"3141592653589793238462643383279502884197169399375105820974944592",
            b"2718281828459045235360287471352662497757247093699959574966967627",
            b"8539734222673567065463550869546574495034888535765114961879601127067743044893204848617875072216249073013374895871952806582723184"
        ),
    ];

    #[test]
    fn test_karatsuba_small_products() {
        for (x, y, expectation) in &SMALL_PRODUCTS {
            let x = x.to_bigint().unwrap();
            let y = y.to_bigint().unwrap();
            let expectation = expectation.to_bigint().unwrap();
            assert_eq!(karatsuba::multiply(x, y), expectation);
        }
    }

    #[test]
    fn test_karatsuba_large_products() {
        for (x, y, expectation) in &BIG_PRODUCTS {
            let x = BigInt::parse_bytes(x, 10).unwrap();
            let y = BigInt::parse_bytes(y, 10).unwrap();
            let expectation = BigInt::parse_bytes(expectation, 10).unwrap();
            assert_eq!(karatsuba::multiply(x, y), expectation);
        }
    }
}