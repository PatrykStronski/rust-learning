use json;
use crate::randoms;

pub fn create() {
    let mut js = json::object!{
        price: randoms::random_generations::generate_random(),
        name: "name of object"
    };
    println!("{}", js);
    make_price_zero(&mut js);
}

fn make_price_zero(js: &mut json::JsonValue) {
    js["price"] = 0.into();
    println!("{}",js["price"]);
    println!("{}", js);
}

