use num_bigint::{
    Sign,
    BigInt
};

fn karatsuba(_x: Vec<u8>, _y: Vec<u8>) -> Vec<u32> {
    vec![1]
}

pub fn multiply(x: BigInt, y: BigInt) -> BigInt {
    let (x_sign, x) = x.to_radix_le(10);
    let (y_sign, y) = y.to_radix_le(10);
    BigInt::new(
        match (x_sign, y_sign) {
            (Sign::Plus, Sign::Plus) => Sign::Plus,
            (Sign::Minus, Sign::Minus) => Sign::Plus,
            _ => Sign::Minus
        },
        karatsuba(x, y)
    )
}