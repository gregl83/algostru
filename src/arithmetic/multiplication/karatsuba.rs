use std::cmp;

use num_integer::Integer;
use num_bigint::BigInt;

/// Karatsuba implementation
fn karatsuba(x: BigInt, y: BigInt) -> BigInt {
    let x_len = x.to_str_radix(10).len() as u32;
    let y_len = y.to_str_radix(10).len() as u32;
    let n = cmp::min(x_len, y_len);

    if n == 1 {
        return x * y;
    }

    let midpoint = n / 2;
    let base = BigInt::from(10);
    let midnum = base.pow(midpoint);

    let a = x.div_floor(&midnum);
    let b = x.mod_floor(&midnum);
    let c = y.div_floor(&midnum);
    let d = y.mod_floor(&midnum);

    let p = &a + &b;
    let q = &c + &d;

    let ac = karatsuba(a, c);
    let bd = karatsuba(b, d);
    let pq = karatsuba(p, q);

    let adbc = &pq - &ac - &bd;

    base.pow(2 * midpoint) * ac + base.pow(midpoint) * adbc + bd
}

/// Karatsuba Multiplication
///
/// Input: two n-digit positive integers x and y
/// Output: product of x * y
/// Assumption: n is power of 2
///
/// ================================================================================================
///
/// n = max length of x or y
///
/// if n equals 1 then
///     base case: return product of x * y
/// else
///     a, b = first and second halves (n/2) of x
///     c, d = first and second halves (n/2) of y
///
///     p = sum of a + b
///     q = sum of c + d
///
///     recursive karatsuba:
///     ac = product of a * c
///     bd = product of b * d
///     pq = product of p * q
///
///     adbc = pq - ac - bd
///
///     return result: 10^n * ac + (10^n/2) * adbc + bd
pub fn multiply(x: BigInt, y: BigInt) -> BigInt {
    karatsuba(x, y)
}