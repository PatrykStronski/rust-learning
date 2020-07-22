fn main() {
    use std::collections::HashMap;

    let text = "ust has a focus on safety and speed. It accomplishes these goals through many ‘zero-cost abstractions’, which means that in Rust, abstractions cost as little as possible in order to make them work. The ownership system is a prime example of a zero cost abstraction. All of the analysis we’ll talk about in this guide is done at compile time. You do not pay any run-time cost for any of these features";
    let mut letters_counter: HashMap<char, u8> = HashMap::new();
    
    for char in text.chars() {
        let mut curr_count: u8 = 0;
        if letters_counter.contains_key(&char) {
            curr_count = *letters_counter.get(&char).unwrap();
        } else {
            curr_count = 0;
        }

        letters_counter.insert(char, curr_count + 1);
    }

    let mut chars: Vec<&char> = letters_counter.keys().collect();
    chars.sort();

    for char in chars {
        println!("{} occurs {} times", char, letters_counter.get(char).unwrap());
    }
}
