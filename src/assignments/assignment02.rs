//! Assignment 2: Mastering common programming concepts (1/2).
//!
//! The primary goal of this assignment is to re-learn the common programming concepts in Rust, especially those in the Rust Book chapters 3 and 5.
//! Please make sure you're comfortable with the concepts to proceed on to the next assignments.
//!
//! You should fill out the `todo!()` placeholders in such a way that `/scripts/grade-02.sh` works fine.
//! See `assignment02_grade.rs` and `/scripts/grade-02.sh` for the test script.

use std::ops::Mul;

const FAHRENHEIT_OFFSET: f64 = 32.0;
const FAHRENHEIT_SCALE: f64 = 5.0 / 9.0;

/// Converts Fahrenheit to Celsius temperature degree.
pub fn fahrenheit_to_celsius(degree: f64) -> f64 {
    (degree - FAHRENHEIT_OFFSET) * FAHRENHEIT_SCALE
}

/// Capitalizes English alphabets (leaving the other characters intact).
pub fn capitalize(input: String) -> String {
    input.to_ascii_uppercase()
}

/// Returns the sum of the given array. (We assume the absence of integer overflow.)
pub fn sum_array(input: &[u64]) -> u64 {
    input.iter().sum()
}

/// Given a non-negative integer, say `n`, return the smallest integer of the form `3^m` that's greater than or equal to `n`.
///
/// For instance, up3(6) = 9, up3(9) = 9, up3(10) = 27. (We assume the absence of integer overflow.)
pub fn up3(n: u64) -> u64 {
    let mut ans: u64 = 1;
    while ans < n {
        ans *= 3;
    }
    ans
}

/// Returns the greatest common divisor (GCD) of two non-negative integers. (We assume the absence of integer overflow.)
pub fn gcd(lhs: u64, rhs: u64) -> u64 {
    let mut a: u64 = lhs;
    let mut b: u64 = rhs;
    loop {
        let q: u64 = a / b;
        let r: u64 = a % b;
        if r != 0 {
            a = b;
            b = r;
        } else {
            break b;
        }
    }
}

/// Returns the array of nC0, nC1, nC2, ..., nCn, where nCk = n! / (k! * (n-k)!). (We assume the absence of integer overflow.)
///
/// Consult <https://en.wikipedia.org/wiki/Pascal%27s_triangle> for computation of binomial coefficients without integer overflow.
pub fn chooses(n: u64) -> Vec<u64> {
    let mut vec: Vec<u64> = Vec::new();
    if n > 0 {
        let mut prev: u64 = 0;
        for i in chooses(n - 1) {
            vec.push(prev + i);
            prev = i;
        }
    }
    vec.push(1);
    vec
}

/// Returns the "zip" of two vectors.
///
/// For instance, `zip(vec![1, 2, 3], vec![4, 5])` equals to `vec![(1, 4), (2, 5)]`.
/// Here, `3` is ignored because it doesn't have a partner.
pub fn zip(lhs: Vec<u64>, rhs: Vec<u64>) -> Vec<(u64, u64)> {
    let mut liter = lhs.iter();
    let mut riter = rhs.iter();
    let mut zipped: Vec<(u64, u64)> = Vec::new();
    while let (Some(&l), Some(&r)) = (liter.next(), riter.next()) {
        zipped.push((l, r))
    }
    zipped
}

/// 2x2 matrix of the following configuration:
///
/// a, b
/// c, d
#[derive(Debug, Clone, Copy)]
struct Mat2 {
    a: u64,
    b: u64,
    c: u64,
    d: u64,
}

/// 2x1 matrix of the following configuration:
///
/// a
/// b
#[derive(Debug, Clone, Copy)]
struct Vec2 {
    a: u64,
    b: u64,
}

impl Mat2 {
    /// Creates an identity matrix.
    fn new() -> Self {
        Self {
            a: 1,
            b: 0,
            c: 0,
            d: 1,
        }
    }
}

impl Mul<Mat2> for Mat2 {
    type Output = Mat2;

    fn mul(self, rhs: Mat2) -> Self::Output {
        Self::Output {
            a: self.a * rhs.a + self.b * rhs.c,
            b: self.a * rhs.b + self.b * rhs.d,
            c: self.c * rhs.a + self.d * rhs.c,
            d: self.c * rhs.b + self.d * rhs.d,
        }
    }
}

impl Mul<Vec2> for Mat2 {
    type Output = Vec2;

    fn mul(self, rhs: Vec2) -> Self::Output {
        Self::Output {
            a: self.a * rhs.a + self.b * rhs.b,
            b: self.c * rhs.a + self.d * rhs.b,
        }
    }
}

impl Mat2 {
    /// Calculates the power of matrix.
    fn power(self, power: u64) -> Mat2 {
        let mut mat: Mat2 = Mat2::new();
        for i in 0..power {
            mat = mat * self;
        }
        mat
    }
}

impl Vec2 {
    /// Gets the upper value of vector.
    fn get_upper(self) -> u64 {
        self.a
    }
}

/// The matrix used for calculating Fibonacci numbers.
const FIBONACCI_MAT: Mat2 = Mat2 {
    a: 1,
    b: 1,
    c: 1,
    d: 0,
};

/// The vector used for calculating Fibonacci numbers.
const FIBONACCI_VEC: Vec2 = Vec2 { a: 1, b: 0 };

/// Calculates the Fibonacci number. (We assume the absence of integer overflow.)
///
/// Consult <https://web.media.mit.edu/~holbrow/post/calculating-fibonacci-numbers-with-matrices-and-linear-algebra/> for matrix computation of Fibonacci numbers.
pub fn fibonacci(n: u64) -> u64 {
    (FIBONACCI_MAT.power(n) * FIBONACCI_VEC).get_upper()
}

/// Writes down the lyrics of "twelve days of christmas".
///
/// Hint: Google the song title for lyrics and look at the test code for the expected result.
pub fn twelve_days_of_christmas_lyrics() -> String {
    const HEAD: &str = " day of Christmas, my true love sent to me";
    const LINES: [&str; 12] = [
        "Twelve drummers drumming",
        "Eleven pipers piping",
        "Ten lords a-leaping",
        "Nine ladies dancing",
        "Eight maids a-milking",
        "Seven swans a-swimming",
        "Six geese a-laying",
        "Five gold rings (five golden rings)",
        "Four calling birds",
        "Three French hens",
        "Two turtledoves",
        "And a partridge in a pear tree",
    ];
    const DAYS: [&str; 12] = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];
    let mut s: String = String::new();
    for (day, day_str) in DAYS.iter().enumerate() {
        s.push_str("On the ");
        s.push_str(day_str);
        s.push_str(HEAD);
        s.push('\n');
        if day == 0 {
            s.push_str("A partridge in a pear tree\n");
        } else {
            for (index, line) in LINES.iter().enumerate().skip(11 - day) {
                if day == 10 && index == 1 {
                    s.push_str("I sent eleven pipers piping")
                } else {
                    s.push_str(line);
                }
                s.push('\n');
            }
        }
        s.push('\n');
    }
    s.push_str(LINES[11]);
    s.push('\n');
    s
}
