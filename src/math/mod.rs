pub mod multiplication;

#[derive(Debug, PartialEq)]
pub enum Sign {
    Positive,
    Negative,
}

pub fn bytes_to_ints(chars: &[u8]) -> (Sign, Vec<u8>) {
    let mut offset: usize = 0;
    let sign = match chars[offset] as char {
        '-' => {
            offset = 1;
            Sign::Negative
        },
        '+' => {
            offset = 1;
            Sign::Positive
        },
        _ => Sign::Positive,
    };

    let mut vec: Vec<u8> = vec![];
    for c in &chars[offset..] {
        let integer = *c as char;
        vec.push(
            integer.to_digit(10).unwrap() as u8
        );
    }

    (sign, vec)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bytes_to_ints_unsigned() {
        let bytes = b"3141592653589793238462643383279502884197169399375105820974944592";
        let expectation = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5, 8, 9, 7, 9, 3, 2, 3, 8, 4, 6, 2, 6, 4, 3, 3, 8, 3, 2, 7, 9, 5, 0, 2, 8, 8, 4, 1, 9, 7, 1, 6, 9, 3, 9, 9, 3, 7, 5, 1, 0, 5, 8, 2, 0, 9, 7, 4, 9, 4, 4, 5, 9, 2];
        let (sign, ints) = bytes_to_ints(bytes);
        assert_eq!(sign, Sign::Positive);
        assert_eq!(ints, expectation);
    }

    #[test]
    fn test_bytes_to_ints_signed() {
        let bytes = b"-2718281828459045235360287471352662497757247093699959574966967627";
        let expectation = vec![2, 7, 1, 8, 2, 8, 1, 8, 2, 8, 4, 5, 9, 0, 4, 5, 2, 3, 5, 3, 6, 0, 2, 8, 7, 4, 7, 1, 3, 5, 2, 6, 6, 2, 4, 9, 7, 7, 5, 7, 2, 4, 7, 0, 9, 3, 6, 9, 9, 9, 5, 9, 5, 7, 4, 9, 6, 6, 9, 6, 7, 6, 2, 7];
        let (sign, ints) = bytes_to_ints(bytes);
        assert_eq!(sign, Sign::Negative);
        assert_eq!(ints, expectation);
    }
}