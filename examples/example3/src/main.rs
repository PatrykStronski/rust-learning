fn main() {
    let stri = String::from("Hello, world!");
    //option one
    for (ind, elem) in stri.chars().enumerate() {
        println!("char at position {}: {} has code {}", ind, elem, elem as u8);
    }
    //option two
    let stri_chars = stri.chars();
    for elem in stri_chars  {
        println!("char {} has code {}", elem, elem as u8);
    }
}
