use num_bigint::{
    Sign,
    BigInt
};

fn karatsuba(x: Vec<u8>, _y: Vec<u8>) -> Vec<u8> {
    x
}

pub fn multiply(x: &[u8], y: &[u8]) -> (Sign, Vec<u8>) {
    let (x_sign, x) = BigInt::parse_bytes(x, 10).unwrap().to_radix_le(10);
    let (y_sign, y) = BigInt::parse_bytes(y, 10).unwrap().to_radix_le(10);
    (
        match (x_sign, y_sign) {
            (Sign::Plus, Sign::Plus) | (Sign::Minus, Sign::Minus) => Sign::Plus,
            _ => Sign::Minus
        },
        karatsuba(x, y)
    )
}