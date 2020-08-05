use std::ops;
use std::fmt::{Display, Debug, Formatter, Result};

#[derive(Copy,Clone)]
struct ComplexNumber {
    re: i16,
    im: i16,
}

impl ops::Add<ComplexNumber> for ComplexNumber {
    type Output = ComplexNumber;
    fn add(self, number2: ComplexNumber) -> ComplexNumber {
        ComplexNumber { re: self.re + number2.re, im: self.im + number2.im }
    }
}

impl ops::Sub for ComplexNumber {
    type Output = ComplexNumber;
    fn sub(self, number2: ComplexNumber) -> ComplexNumber {
        ComplexNumber { re: self.re - number2.re, im: self.im - number2.im }
    }
}

impl ops::AddAssign for ComplexNumber {
    fn add_assign(&mut self, number2: ComplexNumber) {
        self.re += number2.re;
        self.im += number2.im;
    }
}

impl ops::SubAssign for ComplexNumber {
    fn sub_assign(&mut self, number2: ComplexNumber) {
        self.re -= number2.re;
        self.im -= number2.im;
    }
}

impl Debug for ComplexNumber {
    fn fmt(&self, f: &mut Formatter) -> Result {
        if self.im > 0 {
            return write!(f, "{} + {}i", self.re, self.im);
        }
        if self.im == 0 {
            return write!(f, "{}", self.re);
        }
        return write!(f, "{} {}i", self.re, self.im);
    }
}

impl Display for ComplexNumber {
    fn fmt(&self, f: &mut Formatter) -> Result {
        f.write_str(&format!("Re={} Im={}", self.re, self.im))
    }
}

fn main() {
    let c1 = ComplexNumber { re: 2, im: 5 };
    let c2 = ComplexNumber { re: 1, im: 3 };
    println!("{:?}", c1 - c2);
    println!("{:?}", c1 + c2);
    let mut c3 = ComplexNumber { re: 1, im: -3 };
    c3 -= c1;
    println!("{:?}", c3);
    println!("{}", c3);
}
