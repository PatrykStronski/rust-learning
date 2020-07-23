struct MyStr {
    field8: String
}

struct MyStruct {
    field1: u8,
    field2: String,
    field3: MyStr
}

fn main() {
    let ms = MyStruct {
        field1: 4,
        field2: String::from("My String"),
        field3: MyStr {
            field8: String::from("My String2")
        }
    };
    println!("{}, {}", ms.field1, ms.field3.field8);
}
