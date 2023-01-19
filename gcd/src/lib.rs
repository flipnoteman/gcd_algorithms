/// Returns the whole number part of a decimal number
fn integer_part(a: u32, b: u32) -> u32 {
    ((a as f32 / b as f32).floor()) as u32
}

/// Calculates the Greatest Common Denominator of two whole numbers using a bruteforce v1 algorithm
pub fn bruteforce_v1_gcd(lhs: u32, rhs: u32) -> u32 {
    let mut gcd = 0;
    let mut a = lhs;
    let mut b = rhs;
    if a < b {
        // Preprocess to swap the sides if left hand side is not greater than right
        let t = a;
        a = b;
        b = t;
    }

    // checks each number from 1-lhs (because it will always be smaller or equivelant to the left side, no need to check higher than that)
    for i in 1..b {
        if a % i == 0 && b % i == 0 {
            gcd = i;
        }
    }
    gcd
}

/// Calculates the Greatest Common Denominator of two whole numbers using a bruteforce v2 algorithm
pub fn bruteforce_v2_gcd(lhs: u32, rhs: u32) -> u32 {
    let gcd;
    let mut a = lhs;
    let mut b = rhs;
    if a < b {
        // Preprocess to swap the sides if left hand side is not greater than right
        let t = a;
        a = b;
        b = t;
    }

    // checks each number from rhs (because it is always going to be smaller or equal to the left hand side), then trys every number counting downwards until a gcd is found
    let mut i = rhs;
    while !(a % i == 0 && b % i == 0) {
        i -= 1;
    }
    gcd = i;
    gcd
}

/// Calculates the Greatest Common Denominator of two whole numbers using Euclid's Algorithm v1
pub fn euclid_gcd(lhs: u32, rhs: u32) -> u32 {
    let mut remainder: u32 = 1;
    let mut a = lhs;
    let mut b = rhs;

    if a < b {
        // Preprocess to swap the sides if left hand side is not greater than right
        let t = a;
        a = b;
        b = t;
    }

    // Instead of bruteforcing a gcd, relies on finding the highest number with the remainder 0 by reducing the left and right hand sides until the calculated remainder == 0
    while 0 != remainder {
        let quotient = integer_part(a, b);
        remainder = a - quotient * b;
        a = b;
        b = remainder;
    }
    a
}

/// Calculates the Greatest Common Denominator of two whole numbers using Euclid's Algorithm v2
pub fn euclid_v2_gcd(lhs: u32, rhs: u32) -> u32 {
    let mut remainder: u32 = 1;
    let mut a = lhs;
    let mut b = rhs;

    if a < b {
        // Preprocess to swap the sides if left hand side is not greater than right
        let t = a;
        a = b;
        b = t;
    }

    // Tries to eliminate a division by checking for the most common gcd's first (1-3), then executing a similar equation to that of euclids first version of this algorithm
    while 0 != remainder {
        remainder = a - b;
        if remainder >= b {
            remainder -= b;
            if remainder >= b {
                remainder -= b;
                if remainder >= b {
                    remainder = a - b * integer_part(a, b);
                }
            }
        }
        a = b;
        b = remainder;
    }
    a
}
