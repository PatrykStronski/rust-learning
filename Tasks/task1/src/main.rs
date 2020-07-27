fn main() {
    use std::collections::HashMap;

    let text = "Rust has a focus on safety and speed. It accomplishes these goals through many ‘zero-cost abstractions’, which means that in Rust, abstractions cost as little as possible in order to make them work. The ownership system is a prime example of a zero cost abstraction. All of the analysis we’ll talk about in this guide is done at compile time. You do not pay any run-time cost for any of these features";
    let mut letters_counter: HashMap<char, u8> = HashMap::new();
    let vec1: Vec<char> = text.chars().flat_map(char::to_lowercase).collect();
    for single_char in vec1 {
        let curr_count: u8;
        if letters_counter.contains_key(&single_char) {
            curr_count = *letters_counter.get(&single_char).unwrap_or(7);
        } else {
            curr_count = 0;
        }

        letters_counter.insert(single_char, curr_count + 1);
    }

    let mut chars: Vec<&char> = letters_counter.keys().collect();
    chars.sort();

    for char1 in chars { // here don't recommend using keyword as name of variable
        println!("{} occurs {} times", char1, letters_counter.get(char1).unwrap());
    }
}
