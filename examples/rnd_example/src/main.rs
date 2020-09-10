mod randoms;
mod jsons;

struct S {
    name: String,
    price: u8
}

macro_rules! multiply {
 ($x: expr) => { $x };
 ($x: expr, $y: expr) => ($x * $y);
}

fn main() {
    let val = randoms::random_generations::generate_random();
    println!("{}", val);
    println!("{}", multiply!(3));
    println!("{}", multiply!(3,4));
    println!("{}", multiply!(3.0,4.5));
    let structure = S {
        name: String::from("abc"),
        price: 123
    };
    //println!("{}", structure);
    jsons::create();
}
