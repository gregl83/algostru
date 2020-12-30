use crate::math::{
    Sign,
    bytes_to_ints
};

fn karatsuba(x: Vec<u8>, _y: Vec<u8>) -> Vec<u8> {
    x
}

pub fn multiply(x: &[u8], y: &[u8]) -> (Sign, Vec<u8>) {
    let (x_sign, x) = bytes_to_ints(x);
    let (y_sign, y) = bytes_to_ints(y);
    (
        match (x_sign, y_sign) {
            (Sign::Positive, Sign::Positive) | (Sign::Negative, Sign::Negative) => Sign::Positive,
            _ => Sign::Negative
        },
        karatsuba(x, y)
    )
}