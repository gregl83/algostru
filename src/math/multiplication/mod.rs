pub mod karatsuba;

#[cfg(test)]
mod tests {
    use super::*;

    use num_bigint::{BigInt, Sign};

    const PRODUCTS: [(&[u8], &[u8], &[u8]); 9] = [
        (b"10", b"10", b"100"),
        (b"-10", b"10", b"-100"),
        (b"10", b"-10", b"-100"),
        (b"-10", b"-10", b"-100"),
        (b"25", b"25", b"625"),
        (b"-25", b"25", b"-625"),
        (b"25", b"-25", b"-625"),
        (b"-25", b"-25", b"-625"),
        (
            b"3141592653589793238462643383279502884197169399375105820974944592",
            b"2718281828459045235360287471352662497757247093699959574966967627",
            b"8539734222673567065463550869546574495034888535765114961879601127067743044893204848617875072216249073013374895871952806582723184"
        ),
    ];

    #[test]
    fn test_karatsuba_products() {
        for (x, y, expectation) in &PRODUCTS {
            let (sign_expectation, mut product_expectation) = BigInt::parse_bytes(expectation, 10).unwrap().to_radix_le(10);
            if sign_expectation == Sign::Minus {
                product_expectation.push(45);
            }
            let product = karatsuba::multiply(x, y);
            assert_eq!(product, product_expectation);
        }
    }
}