mod complex_number;

fn main() {
    let c1 = complex_number::ComplexNumber { re: 2, im: 5 };
    let c2 = complex_number::ComplexNumber { re: 1, im: 3 };
    println!("{:?}", c1 - c2);
    println!("{:?}", c1 + c2);
    let mut c3 = complex_number::ComplexNumber { re: 1, im: -3 };
    c3 -= c1;
    println!("{:?}", c3);
    println!("{}", c3);
}
