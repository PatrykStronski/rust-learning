use std::fs::File;
use std::io::prelude::*;
use std::io::Result;


fn main() -> Result<()> {
    let mut file = File::open("test.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    println!("{}", contents);
    file.write_all(b"New content")?;
    Ok(())
}
