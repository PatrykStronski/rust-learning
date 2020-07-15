fn ptr_borrowing() {
    let mut integer = 10u8;
    println!("{} and this is a reference {}", integer, &integer);
    let mut integer2 = 22u8;
    let mut integer_ptr: &u8 = &integer2;
    println!("integer_ptr is {}", integer_ptr);
    //integer2 = 128; //can be done only in unsafe mode with raw pointers prefixed with *
    println!("integer_ptr is {}", integer_ptr);
}

fn array_slice() -> [u8; 2] {
    let mut int_array: [u8; 5] = [1,2,3,4,5];
    println!("whole array {:?}", int_array);
    println!("slice with indices 1-3 {:?}", &int_array[1..3]);
    let mut arr_slice = &mut int_array[1..3];
    arr_slice[0] = 40; 
    //println!("whole array after the change {:?}", int_array);
    let mut slice_to_return: [u8;2] = Default::default();
    //slice_to_return.clone_from_slice(arr_slice);
    return slice_to_return; 
}

fn string_slice() {
    let string_a = String::from("new string");
    println!("{} ", string_a);
    let str_slice = &string_a[0..5];
    println!("{}", str_slice);
}

fn main() {
    ptr_borrowing();
    let modified_slice = array_slice();
    println!("modified array {:?}", modified_slice);
    string_slice();
}

