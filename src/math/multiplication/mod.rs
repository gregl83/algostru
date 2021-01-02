pub mod karatsuba;
pub mod strassen;

#[cfg(test)]
mod tests {
    use super::*;

    use num_bigint::{BigInt};

    const PRODUCTS: [(&[u8], &[u8], &[u8]); 6] = [
        (b"10", b"10", b"100"),
        (b"134", b"134", b"17956"),
        (b"5678", b"5678", b"32239684"),
        (b"30000", b"30000", b"900000000"),
        (b"12345678", b"12345678", b"152415765279684"),
        (
            b"3141592653589793238462643383279502884197169399375105820974944592",
            b"2718281828459045235360287471352662497757247093699959574966967627",
            b"8539734222673567065463550869546574495034888535765114961879601127067743044893204848617875072216249073013374895871952806582723184"
        ),
    ];

    #[test]
    fn test_karatsuba_products() {
        for (x, y, expectation) in &PRODUCTS {
            let x = BigInt::parse_bytes(x, 10).unwrap();
            let y = BigInt::parse_bytes(y, 10).unwrap();
            let expectation = BigInt::parse_bytes(expectation, 10).unwrap();
            let product = karatsuba::multiply(x, y);
            assert_eq!(product, expectation);
        }
    }

    #[test]
    fn test_strassen_products() {
        assert_eq!(true, false);
    }
}