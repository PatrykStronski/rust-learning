fn is_prime(unit: &u8) -> bool {
    if unit < &2 {
        return false;
    }
    let unit_sqrt = (*unit as f32).sqrt() as u8;
    for i in 2..(unit_sqrt+1) {
        if unit % i == 0 {
            return false;
        }
    }
    return true;
}

fn make_zero_if_prime(unit: &mut u8) {
    let tmp: u8 = 0; 
    if is_prime(unit) {
        *unit = tmp;
    }
}

fn main() {
    let mut long_vec: Vec<u8> = vec![1,6,8,13,9,44,2,3,83];
    for mut elem in &mut long_vec {
        make_zero_if_prime(&mut elem);
    }
    println!("{:?}", long_vec);
}
