use num_bigint::{
    Sign,
    BigInt
};

/*
    # Base case
    if X < 10 and Y < 10:
        return X * Y

    # determine the size of X and Y
    size = max(len(str(X)), len(str(Y)))

    # Split X and Y
    n = ceil(size // 2)
    p = 10 ** n
    a = floor(X // p)
    b = X % p
    c = floor(Y // p)
    d = Y % p

    # Recur until base case
    ac = karatsuba(a, c)
    bd = karatsuba(b, d)
    e = karatsuba(a + b, c + d) - ac - bd

    # return the equation
    return int(10 ** (2 * n) * ac + (10 ** n) * e + bd)
 */

fn karatsuba(x: Vec<u8>, y: Vec<u8>) -> Vec<u8> {
    if x.len() == 1 && y.len() == 1 {
        println!("base");
    }
    x
}

pub fn multiply(x: &[u8], y: &[u8]) -> Vec<u8> {
    let (x_sign, x) = BigInt::parse_bytes(x, 10).unwrap().to_radix_le(10);
    let (y_sign, y) = BigInt::parse_bytes(y, 10).unwrap().to_radix_le(10);
    let mut product = karatsuba(x, y);
    if (x_sign, y_sign) == (Sign::Plus, Sign::Minus) || (x_sign, y_sign) == (Sign::Minus, Sign::Plus) {
        product.push(45);
    }
    product
}